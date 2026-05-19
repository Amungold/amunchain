use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Full, Limited};
use bytes::Bytes;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use amun_rpc::methods::RpcHandler;
use amun_rpc::types::RpcRequest;

const MAX_CONNECTIONS: usize = 512;
const MAX_RPC_BODY_BYTES: usize = 1_048_576;

fn safe_response(status: StatusCode, body: &str) -> Response<Full<Bytes>> {
    match Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .header("X-Content-Type-Options", "nosniff")
        .header("X-Frame-Options", "DENY")
        .body(Full::new(Bytes::from(body.to_string())))
    {
        Ok(resp) => resp,
        Err(_) => {
            let mut resp = Response::new(Full::new(Bytes::from_static(b"{}")));
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            resp
        }
    }
}

#[derive(Clone)]
pub struct RequestHandler {
    pub rpc: Arc<Mutex<RpcHandler>>,
}

impl Service<Request<Incoming>> for RequestHandler {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let rpc = Arc::clone(&self.rpc);
        Box::pin(async move {
            let auth_header = req
                .headers()
                .get(hyper::header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer ").map(|s| s.to_string()));

            let limited_body = Limited::new(req.into_body(), MAX_RPC_BODY_BYTES);
            let body_bytes = match limited_body.collect().await {
                Ok(collected) => collected.to_bytes(),
                Err(_) => {
                    return Ok(safe_response(StatusCode::PAYLOAD_TOO_LARGE, "{\"error\":\"request too large\"}"));
                }
            };

            let request_str = String::from_utf8_lossy(&body_bytes);

            let rpc_request = match serde_json::from_str::<RpcRequest>(&request_str) {
                Ok(mut req) => {
                    if let Some(token) = auth_header {
                        if let serde_json::Value::Object(ref mut obj) = req.params {
                            obj.insert(
                                "auth_token".to_string(),
                                serde_json::Value::String(token),
                            );
                        }
                    }
                    req
                }
                Err(_) => {
                    return Ok(safe_response(StatusCode::BAD_REQUEST, "{\"error\":\"invalid request\"}"));
                }
            };

            let rpc_response = match rpc.lock() {
                Ok(handler) => handler.handle(&rpc_request),
                Err(_) => {
                    return Ok(safe_response(StatusCode::INTERNAL_SERVER_ERROR, "{\"error\":\"internal error\"}"));
                }
            };

            let response_json = match serde_json::to_string(&rpc_response) {
                Ok(json) => json,
                Err(_) => {
                    return Ok(safe_response(StatusCode::INTERNAL_SERVER_ERROR, "{\"error\":\"serialization failure\"}"));
                }
            };

            Ok(safe_response(StatusCode::OK, &response_json))
        })
    }
}

pub struct HttpServer {
    handler: RequestHandler,
    port: u16,
    tls_acceptor: Option<TlsAcceptor>,
}

impl HttpServer {
    pub fn new(port: u16, rpc_handler: RpcHandler) -> Self {
        Self {
            handler: RequestHandler {
                rpc: Arc::new(Mutex::new(rpc_handler)),
            },
            port,
            tls_acceptor: None,
        }
    }

    pub fn with_tls(mut self, acceptor: TlsAcceptor) -> Self {
        self.tls_acceptor = Some(acceptor);
        self
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        let semaphore = Arc::new(tokio::sync::Semaphore::new(MAX_CONNECTIONS));

        loop {
            let (stream, _) = listener.accept().await?;
            let handler = self.handler.clone();
            let tls = self.tls_acceptor.clone();
            let permit = Arc::clone(&semaphore).acquire_owned().await.ok();

            tokio::spawn(async move {
                let _permit = permit;
                let serve = async {
                    if let Some(acceptor) = tls {
                        if let Ok(tls_stream) = acceptor.accept(stream).await {
                            let _ = hyper::server::conn::http1::Builder::new()
                                .header_read_timeout(Duration::from_secs(5))
                                .keep_alive(true)
                                .serve_connection(hyper_util::rt::TokioIo::new(tls_stream), handler)
                                .await;
                        }
                    } else {
                        let _ = hyper::server::conn::http1::Builder::new()
                            .header_read_timeout(Duration::from_secs(5))
                            .keep_alive(true)
                            .serve_connection(hyper_util::rt::TokioIo::new(stream), handler)
                            .await;
                    }
                };
                let _ = tokio::time::timeout(Duration::from_secs(30), serve).await;
            });
        }
    }
}

use reqwest::{header::HeaderMap, Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::RpcError;

/// JSON-RPC error body we expect from amun-rpc on failure
#[derive(serde::Deserialize)]
struct RpcErrorBody {
    code: Option<i64>,
    message: Option<String>,
}

/// Stateless HTTP client for amun-rpc.
///
/// Does NOT own consensus, storage, or any blockchain state.
/// All data is fetched from the remote RPC endpoint.
#[derive(Clone)]
pub struct RpcClient {
    client: reqwest::Client,
    base_url: Url,
    default_headers: HeaderMap,
}

impl RpcClient {
    // ----------------------------------------------------------------
    // Constructors
    // ----------------------------------------------------------------

    /// Create a new client with the given `base_url` and an already-configured
    /// `reqwest::Client` (timeouts, TLS, proxy, etc. are set externally).
    pub fn new(base_url: Url, client: reqwest::Client) -> Self {
        Self {
            client,
            base_url,
            default_headers: HeaderMap::new(),
        }
    }

    /// Builder-style: attach default headers sent with every request.
    pub fn with_default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = headers;
        self
    }

    /// Return a reference to the base URL.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    // ----------------------------------------------------------------
    // Public wrappers — thin layers over `request()`
    // ----------------------------------------------------------------

    /// Perform a GET request and deserialize the JSON response.
    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, RpcError> {
        self.request::<T, ()>(Method::GET, path, None).await
    }

    /// Perform a POST request with a JSON body and deserialize the response.
    pub async fn post_json<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, RpcError> {
        self.request(Method::POST, path, Some(body)).await
    }

    // ----------------------------------------------------------------
    // Internal — owns the full HTTP lifecycle
    // ----------------------------------------------------------------

    async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, RpcError> {
        // 1. Build URL
        let url = self
            .base_url
            .join(path.trim_start_matches('/'))
            .map_err(|e| RpcError::Rpc {
                http_status: StatusCode::INTERNAL_SERVER_ERROR,
                code: None,
                message: format!("invalid path '{}': {e}", path),
            })?;

        // 2. Create request
        let mut req = self.client.request(method, url);

        // 3. Merge default headers
        for (key, value) in self.default_headers.iter() {
            req = req.header(key, value);
        }

        // 4. Attach JSON body (only for POST / PUT / PATCH)
        if let Some(b) = body {
            req = req.json(b);
        }

        // 5. Send
        let response = req.send().await?;

        // 6. Check HTTP status
        let status = response.status();

        // 7. Validate Content-Type (best-effort)
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if status.is_success() && !content_type.contains("application/json") {
            return Err(RpcError::Rpc {
                http_status: StatusCode::BAD_GATEWAY,
                code: None,
                message: format!(
                    "expected application/json but got '{}'",
                    content_type
                ),
            });
        }

        // TODO: enforce max response size (e.g., 16 MiB) before reading body

        // 8. Read body
        let body_bytes = response.bytes().await?;

        if status.is_success() {
            // 9. Deserialize JSON
            let payload: T = serde_json::from_slice(&body_bytes)?;
            Ok(payload)
        } else {
            // 10. Try to parse structured error body from amun-rpc
            let error_text = match serde_json::from_slice::<RpcErrorBody>(&body_bytes) {
                Ok(rpc_err) => rpc_err
                    .message
                    .unwrap_or_else(|| "no message".to_string()),
                Err(_) => {
                    // Fallback: raw text
                    String::from_utf8_lossy(&body_bytes).into_owned()
                }
            };

            let error_code = serde_json::from_slice::<RpcErrorBody>(&body_bytes)
                .ok()
                .and_then(|e| e.code);

            Err(RpcError::Rpc {
                http_status: status,
                code: error_code,
                message: error_text,
            })
        }
    }
}

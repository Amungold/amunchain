use tokio::net::TcpListener;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;

pub struct WebSocketServer {
    pub port: u16,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        let semaphore = Arc::new(tokio::sync::Semaphore::new(256));

        while let Ok((stream, _)) = listener.accept().await {
            let permit = Arc::clone(&semaphore).acquire_owned().await.ok();

            tokio::spawn(async move {
                let _permit = permit;
                if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
                    let (mut write, mut read) = ws_stream.split();
                    while let Some(Ok(msg)) = read.next().await {
                        if msg.is_close() || msg.is_ping() {
                            if msg.is_ping() {
                                let _ = write
                                    .send(tokio_tungstenite::tungstenite::Message::Pong(vec![]))
                                    .await;
                            }
                            if msg.is_close() {
                                break;
                            }
                            continue;
                        }
                        if msg.is_text() || msg.is_binary() {
                            let response = serde_json::json!({
                                "jsonrpc": "2.0",
                                "result": "subscription_active"
                            })
                            .to_string();
                            let _ = write
                                .send(tokio_tungstenite::tungstenite::Message::Text(response))
                                .await;
                        }
                    }
                }
            });
        }
        Ok(())
    }
}

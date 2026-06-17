use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub peer_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub history_root: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub certificate_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeResponse {
    pub blocks: Vec<BlockResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub rounds_active: usize,
    pub peer_count: usize,
}

pub struct RpcClient {
    host: String,
    port: u16,
}

impl RpcClient {
    pub fn new(host: &str, port: u16) -> Self {
        RpcClient {
            host: host.to_string(),
            port,
        }
    }

    fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))
            .map_err(|e| format!("Connect error: {}", e))?;

        let request = format!(
            "GET /{} HTTP/1.0\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
            path, self.host, self.port
        );
        stream
            .write_all(request.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .map_err(|e| format!("Read error: {}", e))?;

        // Extract body after \r\n\r\n
        let body = response
            .split("\r\n\r\n")
            .nth(1)
            .ok_or_else(|| "No body in response".to_string())?;

        serde_json::from_str(body).map_err(|e| format!("Parse error: {}", e))
    }

    pub fn get_status(&self) -> Result<StatusResponse, String> {
        self.get_json("status")
    }

    pub fn get_head(&self) -> Result<HeadResponse, String> {
        self.get_json("head")
    }

    pub fn get_block(&self, height: u64) -> Result<BlockResponse, String> {
        self.get_json(&format!("block/{}", height))
    }

    pub fn get_block_range(&self, from: u64, to: u64) -> Result<RangeResponse, String> {
        self.get_json(&format!("blocks/{}/{}", from, to))
    }

    pub fn submit_transaction(&self, tx_json: &str) -> Result<String, String> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))
            .map_err(|e| format!("Connect error: {}", e))?;

        let body = tx_json.to_string();
        let request = format!(
            "POST /tx/submit HTTP/1.0
Host: {}:{}
Content-Type: application/json
Content-Length: {}
Connection: close

{}",
            self.host,
            self.port,
            body.len(),
            body
        );
        stream
            .write_all(request.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .map_err(|e| format!("Read error: {}", e))?;

        let body = response
            .split(
                "

",
            )
            .nth(1)
            .ok_or("No body")?;
        let parsed: serde_json::Value =
            serde_json::from_str(body).map_err(|e| format!("Parse error: {}", e))?;
        parsed["hash"]
            .as_str()
            .map(|h| h.to_string())
            .ok_or("Missing hash".to_string())
    }

    pub fn get_metrics(&self) -> Result<MetricsResponse, String> {
        self.get_json("metrics")
    }
}

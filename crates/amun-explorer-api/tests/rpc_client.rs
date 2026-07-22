use amun_explorer_api::error::RpcError;
use amun_explorer_api::rpc::client::RpcClient;

use httpmock::prelude::*;
use reqwest::Url;
use serde_json::json;

// ------------------------------------------------------------------
// Helper: build a mock RpcClient pointing to a mock server
// ------------------------------------------------------------------
fn test_client(server: &MockServer) -> RpcClient {
    let base_url = Url::parse(&server.base_url()).expect("invalid mock url");
    let http_client = reqwest::Client::new();
    RpcClient::new(base_url, http_client)
}

// ==================================================================
// Test 1: Happy path — valid JSON response
// ==================================================================
#[tokio::test]
async fn happy_path_get_status() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET).path("/status");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "node_id": "node-1",
                "version": "1.0.0",
                "network": "testnet",
                "uptime_seconds": 3600,
                "connected_peers": 5
            }));
    });

    let client = test_client(&server);
    let result = client.get_status().await;

    mock.assert();
    assert!(result.is_ok(), "expected Ok, got {:?}", result.err());

    let status = result.unwrap();
    assert_eq!(status.node_id, "node-1");
    assert_eq!(status.version, "1.0.0");
    assert_eq!(status.network, "testnet");
    assert_eq!(status.uptime_seconds, 3600);
    assert_eq!(status.connected_peers, 5);
}

// ==================================================================
// Test 2: HTTP 404 with structured JSON error
// ==================================================================
#[tokio::test]
async fn http_404_with_json_error() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/status");
        then.status(404)
            .header("content-type", "application/json")
            .json_body(json!({
                "code": 1001,
                "message": "not found"
            }));
    });

    let client = test_client(&server);
    let result = client.get_status().await;

    match result {
        Err(RpcError::Rpc {
            http_status,
            code,
            message,
        }) => {
            assert_eq!(http_status, 404);
            assert_eq!(code, Some(1001));
            assert!(message.contains("not found"));
        }
        other => panic!("expected RpcError::Rpc, got {:?}", other),
    }
}

// ==================================================================
// Test 3: HTTP 500 with HTML body (reverse proxy error)
// ==================================================================
#[tokio::test]
async fn http_500_with_html_body() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/status");
        then.status(500)
            .header("content-type", "text/html")
            .body("<html><body>502 Bad Gateway</body></html>");
    });

    let client = test_client(&server);
    let result = client.get_status().await;

    match result {
        Err(RpcError::Rpc {
            http_status,
            code,
            message,
        }) => {
            assert_eq!(http_status, 500);
            assert_eq!(code, None);
            assert!(message.contains("<html>"), "expected HTML in message, got: {message}");
        }
        other => panic!("expected RpcError::Rpc, got {:?}", other),
    }
}

// ==================================================================
// Test 4: 200 OK but wrong Content-Type (e.g., text/html)
// ==================================================================
#[tokio::test]
async fn bad_content_type_rejected() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/status");
        then.status(200)
            .header("content-type", "text/html")
            .body("<html>OK</html>");
    });

    let client = test_client(&server);
    let result = client.get_status().await;

    match result {
        Err(RpcError::Rpc { message, .. }) => {
            assert!(
                message.contains("expected application/json"),
                "unexpected message: {message}"
            );
        }
        other => panic!("expected RpcError::Rpc, got {:?}", other),
    }
}

// ==================================================================
// Test 5: Invalid JSON (200 + application/json + garbage)
// ==================================================================
#[tokio::test]
async fn invalid_json_body() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/status");
        then.status(200)
            .header("content-type", "application/json")
            .body("{this is not valid json}");
    });

    let client = test_client(&server);
    let result = client.get_status().await;

    match result {
        Err(RpcError::Json(_)) => {
            // Expected
        }
        other => panic!("expected RpcError::Json, got {:?}", other),
    }
}

// ==================================================================
// Test 6: URL join — trailing slash in base_url
// ==================================================================
#[test]
fn url_join_with_trailing_slash() {
    let base = Url::parse("http://localhost:8080/api/").unwrap();
    let joined = base.join("status").unwrap();
    assert_eq!(joined.path(), "/api/status");
    assert!(!joined.path().contains("//"), "double slash in path");
}

// ==================================================================
// Test 7: URL join — NO trailing slash in base_url
// ==================================================================
#[test]
fn url_join_no_trailing_slash() {
    let base = Url::parse("http://localhost:8080/api").unwrap();
    // NOTE: Url::join replaces the last segment when base has no trailing slash.
    // "api" is treated as a file, not a directory.
    let joined = base.join("status").unwrap();
    // This will be "/status", NOT "/api/status"
    // Documenting this behavior — if the real RPC URL has no trailing slash,
    // paths will be relative to root, not to /api/.
    assert_eq!(joined.path(), "/status");
}

// ==================================================================
// Test 7b: URL join — base ends with slash, path has leading slash
// ==================================================================
#[test]
fn url_join_both_slashes() {
    let base = Url::parse("http://localhost:8080/api/").unwrap();
    let joined = base.join("/status").unwrap();
    // Leading slash in path should still append to base, not replace it
    assert_eq!(joined.path(), "/status");
}

// ==================================================================
// Test 7c: URL join — base no slash, path no slash
// ==================================================================
#[test]
fn url_join_no_slashes() {
    let base = Url::parse("http://localhost:8080/api").unwrap();
    let joined = base.join("status").unwrap();
    // "api" treated as file, "status" replaces it
    assert_eq!(joined.path(), "/status");
}

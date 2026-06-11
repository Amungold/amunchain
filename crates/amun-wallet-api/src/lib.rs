pub mod errors;
pub mod server;
pub mod types;
pub mod routes {
    pub mod accounts;
    pub mod chain;
    pub mod network;
    pub mod transactions;
}
pub mod services {
    pub mod account_service;
    pub mod chain_service;
    pub mod network_service;
    pub mod transaction_service;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn n48_3_get_network_info() {
        let app = server::build_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/network/info")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn n48_3_get_chain_head() {
        let app = server::build_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/chain/head")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn n48_3_build_transaction() {
        let app = server::build_app();
        let body = json!({"transaction_type":"transfer","sender":"0xaaa","recipient":"0xbbb","amount":1000,"nonce":1});
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/transactions/build")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn n48_3_build_transaction_invalid() {
        let app = server::build_app();
        let body =
            json!({"transaction_type":"transfer","sender":"","recipient":"","amount":0,"nonce":0});
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/transactions/build")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    // Service-level tests (bypass router, test business logic directly)
    #[test]
    fn n48_3_service_get_balance() {
        let result = services::account_service::AccountService::get_balance("0xalice");
        assert!(result.is_ok());
        let account = result.unwrap();
        assert_eq!(account.address, "0xalice");
    }

    #[test]
    fn n48_3_service_get_transaction() {
        let result =
            services::transaction_service::TransactionService::get_transaction("0xdeadbeef");
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.hash, "0xdeadbeef");
        assert_eq!(tx.status, "pending");
    }
}

pub mod errors;
pub mod server;
pub mod types;
pub mod routes {
    pub mod accounts;
    pub mod chain;
    pub mod constitutional;
    pub mod finality;
}
pub mod services {
    pub mod account_service;
    pub mod chain_service;
    pub mod constitutional_service;
    pub mod finality_service;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    fn app() -> axum::Router {
        server::build_app()
    }

    // Static route tests (no path parameters)
    #[tokio::test]
    async fn n48_4_get_chain_head() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/explorer/chain/head")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn n48_4_list_finality_certificates() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/explorer/finality/certificates")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn n48_4_get_constitutional_dashboard() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/explorer/constitutional/dashboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn n48_4_list_verdicts() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/explorer/constitutional/verdicts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn n48_4_list_evidence() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/explorer/constitutional/evidence")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Service-level tests (bypass router, test business logic)
    #[test]
    fn n48_4_service_get_block() {
        let result = services::chain_service::ChainService::get_block_by_height(847);
        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.height, 847);
        assert!(block.has_finality_certificate);
        assert!(block.has_replay_evidence);
    }

    #[test]
    fn n48_4_service_get_transaction() {
        let result = services::chain_service::ChainService::get_transaction("0xdeadbeef");
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.status, "confirmed");
    }

    #[test]
    fn n48_4_service_get_account() {
        let result = services::account_service::AccountService::get_account("0xalice");
        assert!(result.is_ok());
        let account = result.unwrap();
        assert_eq!(account.balance, 100_000);
    }
}

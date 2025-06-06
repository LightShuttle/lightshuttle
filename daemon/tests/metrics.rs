use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use lightshuttle_core::app::build_router;
use tower::ServiceExt;

#[tokio::test]
async fn metrics_works() {
    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

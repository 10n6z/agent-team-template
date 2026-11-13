use axum::{http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};

fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .fallback(not_found)
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok", "service": "backend", "version": env!("CARGO_PKG_VERSION")}))
}

async fn not_found() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_FOUND, Json(json!({"error": "not_found"})))
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("bind {addr}: {e}"));
    println!("backend listening on {addr}");
    axum::serve(listener, app()).await.expect("server error");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{header, Request, StatusCode};
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    async fn get(path: &str) -> (StatusCode, String, Value) {
        let res = app()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = res.status();
        let ct = res
            .headers()
            .get(header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap().to_owned())
            .unwrap_or_default();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        (
            status,
            ct,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }

    #[tokio::test]
    async fn health_ok() {
        let (status, ct, body) = get("/health").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(ct, "application/json");
        assert_eq!(
            body,
            json!({"status": "ok", "service": "backend", "version": env!("CARGO_PKG_VERSION")})
        );
    }

    #[tokio::test]
    async fn unknown_path_is_404_json() {
        let (status, ct, body) = get("/nope").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(ct, "application/json");
        assert_eq!(body, json!({"error": "not_found"}));
    }
}

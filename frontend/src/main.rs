mod api;

use axum::{extract::State, response::Html, routing::get, Json, Router};
use serde_json::{json, Value};

#[derive(Clone)]
struct AppState {
    api: api::Client,
}

fn app(api_url: &str) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .with_state(AppState {
            api: api::Client::new(api_url),
        })
}

async fn index(State(s): State<AppState>) -> Html<String> {
    // Only these two fixed words ever reach the page, so nothing dynamic needs escaping.
    let status = if s.api.backend_ok().await {
        "ok"
    } else {
        "unavailable"
    };
    Html(format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Status</title>
</head>
<body>
<main>
<h1>Status</h1>
<p>Backend status: <span id="backend-status">{status}</span></p>
</main>
</body>
</html>
"#
    ))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok", "service": "frontend", "version": env!("CARGO_PKG_VERSION")}))
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8080".into());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("bind");
    axum::serve(listener, app(&api_url)).await.expect("serve");
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn spawn(router: Router) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        url
    }

    async fn get_page(api_url: &str) -> (u16, String, String) {
        let fe = spawn(app(api_url)).await;
        let r = reqwest::get(format!("{fe}/")).await.unwrap();
        let ct = r.headers()["content-type"].to_str().unwrap().to_owned();
        (r.status().as_u16(), ct, r.text().await.unwrap())
    }

    fn fake_backend(status: axum::http::StatusCode, body: &'static str) -> Router {
        Router::new().route("/health", get(move || async move { (status, body) }))
    }

    #[tokio::test]
    async fn index_ok_when_backend_healthy() {
        let be = spawn(fake_backend(
            axum::http::StatusCode::OK,
            r#"{"status":"ok","service":"backend","version":"0.1.0"}"#,
        ))
        .await;
        let (code, ct, body) = get_page(&be).await;
        assert_eq!(code, 200);
        assert_eq!(ct, "text/html; charset=utf-8");
        assert!(body.starts_with("<!doctype html>"));
        assert!(body.contains(r#"<html lang="en">"#));
        assert!(body.contains("<title>"));
        assert_eq!(body.matches(r#"<span id="backend-status">"#).count(), 1);
        assert!(body.contains(r#"<span id="backend-status">ok</span>"#));
    }

    #[tokio::test]
    async fn index_unavailable_when_backend_unreachable() {
        // Bind then drop to get a port nothing is listening on.
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let (code, _, body) = get_page(&format!("http://127.0.0.1:{port}")).await;
        assert_eq!(code, 200);
        assert!(body.contains(r#"<span id="backend-status">unavailable</span>"#));
    }

    #[tokio::test]
    async fn index_unavailable_on_bad_backend_responses() {
        use axum::http::StatusCode;
        for (status, body) in [
            (StatusCode::INTERNAL_SERVER_ERROR, r#"{"status":"ok"}"#),
            (StatusCode::OK, "not json"),
            (StatusCode::OK, r#"{"status":"degraded"}"#),
        ] {
            let be = spawn(fake_backend(status, body)).await;
            let (code, _, page) = get_page(&be).await;
            assert_eq!(code, 200);
            assert!(
                page.contains(r#"<span id="backend-status">unavailable</span>"#),
                "{status} {body}"
            );
        }
    }

    #[tokio::test]
    async fn index_unavailable_when_backend_hangs() {
        let be = spawn(Router::new().route(
            "/health",
            get(|| async { std::future::pending::<&'static str>().await }),
        ))
        .await;
        let started = std::time::Instant::now();
        let (code, _, page) = get_page(&be).await;
        assert_eq!(code, 200);
        assert!(page.contains(r#"<span id="backend-status">unavailable</span>"#));
        assert!(started.elapsed() < std::time::Duration::from_secs(4));
    }

    #[tokio::test]
    async fn health_json() {
        let fe = spawn(app("http://127.0.0.1:1")).await;
        let r = reqwest::get(format!("{fe}/health")).await.unwrap();
        assert_eq!(r.status(), 200);
        assert_eq!(r.headers()["content-type"], "application/json");
        let v: Value = r.json().await.unwrap();
        assert_eq!(
            v,
            json!({"status": "ok", "service": "frontend", "version": env!("CARGO_PKG_VERSION")})
        );
    }

    #[tokio::test]
    async fn unknown_path_404() {
        let fe = spawn(app("http://127.0.0.1:1")).await;
        let r = reqwest::get(format!("{fe}/nope")).await.unwrap();
        assert_eq!(r.status(), 404);
    }
}

//! The only place that talks to the backend (CONTRACTS.md v1, "Backend service").

use std::time::Duration;

#[derive(serde::Deserialize)]
struct Health {
    status: String,
}

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    health_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Client {
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(2))
                .build()
                .expect("http client"),
            health_url: format!("{}/health", base_url.trim_end_matches('/')),
        }
    }

    /// True only for `200` with JSON `"status":"ok"`; any error, timeout or other reply is false.
    pub async fn backend_ok(&self) -> bool {
        let Ok(resp) = self.http.get(&self.health_url).send().await else {
            return false;
        };
        if resp.status() != reqwest::StatusCode::OK {
            return false;
        }
        matches!(resp.json::<Health>().await, Ok(h) if h.status == "ok")
    }
}

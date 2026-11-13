//! Shared helpers for the contract v1 e2e suite.

use std::time::Duration;

pub const DEFAULT_API_URL: &str = "http://localhost:8080";
pub const DEFAULT_FRONTEND_URL: &str = "http://localhost:3000";

/// Env value if set and non-blank, else the default; trailing slashes removed.
pub fn resolve_base_url(value: Option<String>, default: &str) -> String {
    let v = value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| default.to_string());
    v.trim_end_matches('/').to_string()
}

pub fn api_url() -> String {
    resolve_base_url(std::env::var("API_URL").ok(), DEFAULT_API_URL)
}

pub fn frontend_url() -> String {
    resolve_base_url(std::env::var("FRONTEND_URL").ok(), DEFAULT_FRONTEND_URL)
}

pub fn client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("build http client")
}

pub fn content_type(resp: &reqwest::blocking::Response) -> String {
    resp.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}

const STATUS_ATTR: &str = "id=\"backend-status\"";

/// Number of `id="backend-status"` occurrences in the page.
pub fn backend_status_count(html: &str) -> usize {
    html.matches(STATUS_ATTR).count()
}

/// Text of the first `id="backend-status"` element (between its `>` and the next `<`).
pub fn backend_status_text(html: &str) -> Option<&str> {
    let rest = &html[html.find(STATUS_ATTR)? + STATUS_ATTR.len()..];
    let rest = &rest[rest.find('>')? + 1..];
    Some(&rest[..rest.find('<')?])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unset_url_falls_back_to_default() {
        assert_eq!(
            resolve_base_url(None, DEFAULT_API_URL),
            "http://localhost:8080"
        );
    }

    #[test]
    fn blank_url_falls_back_to_default() {
        assert_eq!(
            resolve_base_url(Some("  ".into()), DEFAULT_FRONTEND_URL),
            "http://localhost:3000"
        );
    }

    #[test]
    fn set_url_overrides_default_and_drops_trailing_slashes() {
        assert_eq!(
            resolve_base_url(Some("http://backend:8080//".into()), DEFAULT_API_URL),
            "http://backend:8080"
        );
    }

    #[test]
    fn extracts_backend_status_text_from_span() {
        let html = r#"<p>Backend: <span id="backend-status">ok</span></p>"#;
        assert_eq!(backend_status_count(html), 1);
        assert_eq!(backend_status_text(html), Some("ok"));
    }

    #[test]
    fn extracts_backend_status_with_other_attributes() {
        let html = r#"<span class="s" id="backend-status" data-x="1">unavailable</span>"#;
        assert_eq!(backend_status_text(html), Some("unavailable"));
    }

    #[test]
    fn missing_backend_status_yields_zero_and_none() {
        let html = "<html><body>nothing</body></html>";
        assert_eq!(backend_status_count(html), 0);
        assert_eq!(backend_status_text(html), None);
    }

    #[test]
    fn counts_duplicate_backend_status_elements() {
        let html = r#"<span id="backend-status">ok</span><span id="backend-status">ok</span>"#;
        assert_eq!(backend_status_count(html), 2);
    }
}

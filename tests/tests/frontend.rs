//! Contract v1: Frontend service. Needs a running stack (`make -C tests e2e`).

use e2e::{backend_status_count, backend_status_text, client, content_type, frontend_url};
use serde_json::Value;

#[test]
#[ignore = "e2e: needs running stack"]
fn frontend_health_returns_ok_json_with_exact_keys() {
    let resp = client()
        .get(format!("{}/health", frontend_url()))
        .send()
        .expect("GET frontend /health");
    assert_eq!(resp.status().as_u16(), 200);
    let ct = content_type(&resp);
    assert!(ct.starts_with("application/json"), "content-type: {ct:?}");

    let body: Value = resp.json().expect("JSON body");
    let obj = body.as_object().expect("JSON object");
    let mut keys: Vec<&str> = obj.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["service", "status", "version"], "body: {body}");
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "frontend");
    let version = body["version"].as_str().expect("version is a string");
    assert!(!version.is_empty(), "version must be non-empty");
}

#[test]
#[ignore = "e2e: needs running stack"]
fn frontend_index_shows_backend_status_ok_when_stack_is_up() {
    let resp = client()
        .get(format!("{}/", frontend_url()))
        .send()
        .expect("GET frontend /");
    assert_eq!(resp.status().as_u16(), 200);
    let ct = content_type(&resp);
    assert!(ct.starts_with("text/html"), "content-type: {ct:?}");

    let html = resp.text().expect("HTML body");
    assert_eq!(
        backend_status_count(&html),
        1,
        "expected exactly one id=\"backend-status\""
    );
    assert_eq!(backend_status_text(&html), Some("ok"));
}

#[test]
#[ignore = "e2e: needs running stack"]
fn frontend_unknown_path_returns_404() {
    let resp = client()
        .get(format!("{}/definitely-not-a-route-e2e", frontend_url()))
        .send()
        .expect("GET frontend unknown path");
    assert_eq!(resp.status().as_u16(), 404);
}

//! Contract v1: Backend service. Needs a running stack (`make -C tests e2e`).

use e2e::{api_url, client, content_type};
use serde_json::Value;

#[test]
#[ignore = "e2e: needs running stack"]
fn backend_health_returns_ok_json_with_exact_keys() {
    let resp = client()
        .get(format!("{}/health", api_url()))
        .send()
        .expect("GET backend /health");
    assert_eq!(resp.status().as_u16(), 200);
    let ct = content_type(&resp);
    assert!(ct.starts_with("application/json"), "content-type: {ct:?}");

    let body: Value = resp.json().expect("JSON body");
    let obj = body.as_object().expect("JSON object");
    let mut keys: Vec<&str> = obj.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["service", "status", "version"], "body: {body}");
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "backend");
    let version = body["version"].as_str().expect("version is a string");
    assert!(!version.is_empty(), "version must be non-empty");
}

#[test]
#[ignore = "e2e: needs running stack"]
fn backend_unknown_path_returns_404_not_found_error() {
    let resp = client()
        .get(format!("{}/definitely-not-a-route-e2e", api_url()))
        .send()
        .expect("GET backend unknown path");
    assert_eq!(resp.status().as_u16(), 404);
    let body: Value = resp.json().expect("JSON error body");
    assert_eq!(body, serde_json::json!({"error": "not_found"}));
}

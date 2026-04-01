use reqwest::blocking::{Client, RequestBuilder};
use std::collections::HashMap;

fn build_url(project: &str, path: &str) -> String {
    format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/{}",
        project,
        path.trim_start_matches('/')
    )
}

fn client() -> Client {
    Client::new()
}

fn apply_headers(
    mut builder: RequestBuilder,
    headers: Option<HashMap<String, String>>,
) -> RequestBuilder {
    if let Some(headers) = headers {
        for (k, v) in &headers {
            builder = builder.header(k.as_str(), v.as_str());
        }
    }
    builder
}

pub fn core_get(
    auth_token: &str,
    project: &str,
    path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let builder = client()
        .get(build_url(project, path))
        .header("Authorization", format!("Bearer {auth_token}"));
    let builder = apply_headers(builder, headers);
    let response = builder.send().map_err(|e| format!("Request failed: {e}"))?;
    let status = response.status();
    let text = response
        .text()
        .map_err(|e| format!("Failed to read response: {e}"))?;

    if !status.is_success() {
        return Err(format!("HTTP {status}: {text}"));
    }
    Ok(text)
}

pub fn core_patch(
    auth_token: &str,
    project: &str,
    path: &str,
    field_type: &str,
    field_path: &str,
    field_value: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let url = format!(
        "{}?updateMask.fieldPaths={}",
        build_url(project, path),
        field_path
    );
    let value = match field_type {
        "integerValue" => {
            serde_json::json!({"integerValue": field_value})
        }
        "doubleValue" => {
            serde_json::json!({"doubleValue": field_value.parse::<f64>().map_err(|_| format!("Invalid double: {field_value}"))})
        }
        "booleanValue" => match field_value {
            "true" => {
                serde_json::json!({"booleanValue": true})
            }
            "false" => {
                serde_json::json!({"booleanValue": false})
            }
            _ => {
                return Err(format!("Invalid boolean: {field_value}"));
            }
        },
        "stringValue" => {
            serde_json::json!({"stringValue": field_value})
        }
        "nullValue" => {
            serde_json::json!({"nullValue": null})
        }
        "timestampValue" => {
            chrono::DateTime::parse_from_rfc3339(field_value).map_err(|_| format!(
            "Invalid timestampValue: '{}' must be RFC3339 format e.g. 2026-03-19T15:37:04.272Z",
            field_value
        ))?;
            serde_json::json!({ "timestampValue": field_value })
        }
        _ => {
            return Err(format!("Invalid field type: {field_type}"));
        }
    };

    let fields = build_field(field_path, value);
    let body = serde_json::json!({"fields": fields});
    let builder = client()
        .patch(&url)
        .header("Authorization", format!("Bearer {auth_token}"))
        .header("Content-Type", "application/json")
        .body(body.to_string());

    let builder = apply_headers(builder, headers);
    let response = builder.send().map_err(|e| format!("Request failed: {e}"))?;
    let status = response.status();
    let text = response
        .text()
        .map_err(|e| format!("Failed to read response: {e}"))?;

    if !status.is_success() {
        return Err(format!("HTTP {status}: {text}"));
    }

    Ok(text)
}

fn build_field(field_path: &str, value: serde_json::Value) -> serde_json::Value {
    let parts: Vec<&str> = field_path.splitn(2, '.').collect();
    if parts.len() == 1 {
        serde_json::json!({ parts[0]: value })
    } else {
        let inner = build_field(parts[1], value);
        serde_json::json!({
            parts[0]: { "mapValue": { "fields": inner } }
        })
    }
}

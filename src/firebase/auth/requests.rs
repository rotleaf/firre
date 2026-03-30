use std::collections::HashMap;

use crate::firebase::firebase_error;

pub fn core_refresh_token(
    api_key: &str,
    refresh_token: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://securetoken.googleapis.com/v1/token?key={}",
        api_key
    );
    let mut body = HashMap::new();
    body.insert("grant_type", "refresh_token");
    body.insert("refresh_token", refresh_token);

    let mut builder = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body);

    if let Some(headx) = headers {
        for (k, v) in &headx {
            builder = builder.header(k.as_str(), v.as_str());
        }
    }
    let response = builder.send().map_err(|e| format!("Request failed: {e}"))?;

    let status = response.status();
    let text = response
        .text()
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    if !status.is_success() {
        return Err(format!("HTTP {status}: {text}"));
    }

    let json = serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))?;
    firebase_error(&json)?;

    Ok(text)
}

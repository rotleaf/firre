use reqwest::blocking::{Client, RequestBuilder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

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

/// timestamp value
pub fn core_server_timestamp(
    auth_token: &str,
    project: &str,
    path: &str,
    field_path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let commit_url = format!(
        "https://firestore.googleapis.com/v1/projects/{project}/databases/(default)/documents:commit",
    );
    let body = serde_json::json!({
        "writes": [{
            "transform": {
                "document": format!("projects/{project}/databases/(default)/documents/{}", path.trim_start_matches("/")),
                "fieldTransforms": [{
                    "fieldPath": field_path,
                    "setToServerValue": "REQUEST_TIME"
                }]
            }
        }]
    });

    let builder = client()
        .post(&commit_url)
        .header("Authorization", format!("Bearer {auth_token}"))
        .header("Content-Type", "application/json")
        .body(body.to_string());
    let builder = apply_headers(builder, headers);
    let response = builder.send().map_err(|e| format!("Request failed:{e}"))?;
    let status = response.status();
    let text = response
        .text()
        .map_err(|e| format!("Failed to read response: {e}"))?;

    if !status.is_success() {
        return Err(format!("HTTP {status}: {text}"));
    }

    Ok(text)
}

/// increment numeric fields, integerValue
pub fn core_field_increment(
    auth_token: &str,
    project: &str,
    path: &str,
    field_path: &str,
    amount: f64,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let increment_value = if amount.fract() == 0.0 {
        serde_json::json!({ "integerValue": amount as i64 })
    } else {
        serde_json::json!({ "doubleValue": amount })
    };
    let body = serde_json::json!({
        "writes": [{
            "transform": {
                "document": format!("projects/{project}/databases/(default)/documents/{}", path.trim_start_matches("/")),
                "fieldTransforms": [{
                    "fieldPath": field_path,
                    "increment": increment_value
                }]
            }
        }]
    });

    let builder = client().post(format!("https://firestore.googleapis.com/v1/projects/{project}/databases/(default)/documents:commit")).header("Authorization", format!("Bearer {auth_token}")).header("Content-Type", "application/json").body(body.to_string());
    let builder = apply_headers(builder, headers);
    let response = builder.send().map_err(|e| format!("Request Failed: {e}"))?;
    let status = response.status();
    let text = response
        .text()
        .map_err(|e| format!("Failed to read response: {e}"))?;

    if !status.is_success() {
        return Err(format!("HTTP {status}:{text}"));
    }

    Ok(text)
}

/// delete a field from a document
pub fn core_delete_field(
    auth_token: &str,
    project: &str,
    path: &str,
    field_path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let url = format!(
        "{}?updateMask.fieldPaths={}",
        build_url(project, path),
        field_path
    );
    let body = serde_json::json!({"fields": {}});
    let builder = client()
        .patch(&url)
        .header("Authorization", format!("Bearer {auth_token}"))
        .header("Content-Type", "application-json")
        .body(body.to_string());
    let builder = apply_headers(builder, headers);
    let response = builder.send().map_err(|e| format!("Request Failed: {e}"))?;
    let status = response.status();

    let text = response
        .text()
        .map_err(|_e| "Failed to read response".to_string())?;

    if !status.is_success() {
        return Err(format!("HTTP {status}: {text}"));
    }

    Ok(text)
}

/// get a firestore document
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

/// patch a field in a document
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
            let parsed: f64 = field_value
                .parse()
                .map_err(|_| format!("Invalid double: {field_value}"))?;
            serde_json::json!({"doubleValue": parsed})
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

/// list documents in a collection (name-only, to keep payload small)
pub fn core_list_documents(
    auth_token: &str,
    project: &str,
    collection_path: &str,
    page_size: i32,
    page_token: Option<&str>,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let mut url = format!(
        "{}?pageSize={}&mask.fieldPaths=__name__",
        build_url(project, collection_path),
        page_size
    );
    if let Some(token) = page_token {
        url.push_str(&format!("&pageToken={}", token));
    }

    let builder = client()
        .get(&url)
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

/// batch delete full document names (projects/.../documents/...), max 500 per call
pub fn core_batch_delete(
    auth_token: &str,
    project: &str,
    document_names: &[String],
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let writes: Vec<serde_json::Value> = document_names
        .iter()
        .map(|name| serde_json::json!({ "delete": name }))
        .collect();

    let body = serde_json::json!({ "writes": writes });
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{project}/databases/(default)/documents:batchWrite",
    );

    let builder = client()
        .post(&url)
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

/// delete every document in a collection, paging + batching automatically.
/// returns the total number of documents deleted.
pub fn core_delete_collection(
    auth_token: &str,
    project: &str,
    collection_path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<u32, String> {
    let mut total_deleted: u32 = 0;
    let mut page_token: Option<String> = None;

    loop {
        let list_json = core_list_documents(
            auth_token,
            project,
            collection_path,
            300,
            page_token.as_deref(),
            headers.clone(),
        )?;

        let parsed: serde_json::Value =
            serde_json::from_str(&list_json).map_err(|e| format!("Bad list response: {e}"))?;

        let documents = parsed
            .get("documents")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        if documents.is_empty() {
            break;
        }

        let names: Vec<String> = documents
            .iter()
            .filter_map(|d| d.get("name").and_then(|n| n.as_str()).map(String::from))
            .collect();

        for chunk in names.chunks(500) {
            core_batch_delete(auth_token, project, chunk, headers.clone())?;
            total_deleted += chunk.len() as u32;
        }

        page_token = parsed
            .get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(String::from);

        if page_token.is_none() {
            break;
        }
    }

    Ok(total_deleted)
}

/// list full documents in a collection, handling pagination internally.
/// returns the raw JSON array of document objects (each with name + fields).
pub fn core_get_collection(
    auth_token: &str,
    project: &str,
    collection_path: &str,
    page_size: i32,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let mut all_docs: Vec<serde_json::Value> = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut url = format!(
            "{}?pageSize={}",
            build_url(project, collection_path),
            page_size
        );
        if let Some(token) = &page_token {
            url.push_str(&format!("&pageToken={}", token));
        }

        let builder = client()
            .get(&url)
            .header("Authorization", format!("Bearer {auth_token}"));
        let builder = apply_headers(builder, headers.clone());
        let response = builder.send().map_err(|e| format!("Request failed: {e}"))?;
        let status = response.status();
        let text = response
            .text()
            .map_err(|e| format!("Failed to read response: {e}"))?;

        if !status.is_success() {
            return Err(format!("HTTP {status}: {text}"));
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("Bad list response: {e}"))?;

        if let Some(docs) = parsed.get("documents").and_then(|d| d.as_array()) {
            all_docs.extend(docs.iter().cloned());
        }

        page_token = parsed
            .get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(String::from);

        if page_token.is_none() {
            break;
        }
    }

    serde_json::to_string(&all_docs).map_err(|e| format!("Failed to serialize: {e}"))
}

/// delete a single document
pub fn core_delete_document(
    auth_token: &str,
    project: &str,
    path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let builder = client()
        .delete(build_url(project, path))
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

/// list all document IDs (bare, not full paths) in a collection, paginated internally.
pub fn core_get_document_ids(
    auth_token: &str,
    project: &str,
    collection_path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<Vec<String>, String> {
    let mut ids: Vec<String> = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut url = format!(
            "{}?pageSize=300&mask.fieldPaths=__name__",
            build_url(project, collection_path)
        );
        if let Some(token) = &page_token {
            url.push_str(&format!("&pageToken={}", token));
        }

        let builder = client()
            .get(&url)
            .header("Authorization", format!("Bearer {auth_token}"));
        let builder = apply_headers(builder, headers.clone());
        let response = builder.send().map_err(|e| format!("Request failed: {e}"))?;
        let status = response.status();
        let text = response
            .text()
            .map_err(|e| format!("Failed to read response: {e}"))?;

        if !status.is_success() {
            return Err(format!("HTTP {status}: {text}"));
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("Bad list response: {e}"))?;

        if let Some(docs) = parsed.get("documents").and_then(|d| d.as_array()) {
            for doc in docs {
                if let Some(name) = doc.get("name").and_then(|n| n.as_str()) {
                    // name is full path: projects/.../databases/(default)/documents/coll/id
                    if let Some(id) = name.rsplit('/').next() {
                        ids.push(id.to_string());
                    }
                }
            }
        }

        page_token = parsed
            .get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(String::from);

        if page_token.is_none() {
            break;
        }
    }

    Ok(ids)
}

/// list all documents (full field data) in a collection, paginated internally.
/// returns a Vec of raw JSON strings, one per document.
pub fn core_get_documents(
    auth_token: &str,
    project: &str,
    collection_path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<Vec<String>, String> {
    let mut docs_out: Vec<String> = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut url = format!("{}?pageSize=300", build_url(project, collection_path));
        if let Some(token) = &page_token {
            url.push_str(&format!("&pageToken={}", token));
        }

        let builder = client()
            .get(&url)
            .header("Authorization", format!("Bearer {auth_token}"));
        let builder = apply_headers(builder, headers.clone());
        let response = builder.send().map_err(|e| format!("Request failed: {e}"))?;
        let status = response.status();
        let text = response
            .text()
            .map_err(|e| format!("Failed to read response: {e}"))?;

        if !status.is_success() {
            return Err(format!("HTTP {status}: {text}"));
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("Bad list response: {e}"))?;

        if let Some(docs) = parsed.get("documents").and_then(|d| d.as_array()) {
            for doc in docs {
                docs_out.push(doc.to_string());
            }
        }

        page_token = parsed
            .get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(String::from);

        if page_token.is_none() {
            break;
        }
    }

    Ok(docs_out)
}

/// delete all documents in a collection using concurrent single-document
/// DELETE calls (not batchWrite) — for cases where batchWrite is blocked
/// by rules/permissions but individual deletes are allowed.
pub fn core_delete_collection_concurrent(
    auth_token: &str,
    project: &str,
    collection_path: &str,
    concurrency: usize,
    headers: Option<HashMap<String, String>>,
) -> Result<u32, String> {
    let mut all_names: Vec<String> = Vec::new();
    let mut page_token: Option<String> = None;

    // Phase 1: collect every doc's full path (name-only mask, cheap)
    loop {
        let list_json = core_list_documents(
            auth_token,
            project,
            collection_path,
            300,
            page_token.as_deref(),
            headers.clone(),
        )?;

        let parsed: serde_json::Value =
            serde_json::from_str(&list_json).map_err(|e| format!("Bad list response: {e}"))?;

        if let Some(docs) = parsed.get("documents").and_then(|d| d.as_array()) {
            for doc in docs {
                if let Some(name) = doc.get("name").and_then(|n| n.as_str()) {
                    all_names.push(name.to_string());
                }
            }
        }

        page_token = parsed
            .get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(String::from);

        if page_token.is_none() {
            break;
        }
    }

    // Phase 2: delete concurrently, capped at `concurrency` in-flight threads per batch
    let total_deleted = Arc::new(Mutex::new(0u32));
    let errors: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    for chunk in all_names.chunks(concurrency) {
        let mut handles = Vec::new();

        for name in chunk {
            let auth_token = auth_token.to_string();
            let name = name.clone();
            let headers = headers.clone();
            let total_deleted = Arc::clone(&total_deleted);
            let errors = Arc::clone(&errors);

            handles.push(thread::spawn(move || {
                let url = format!("https://firestore.googleapis.com/v1/{name}");
                let builder = Client::new()
                    .delete(&url)
                    .header("Authorization", format!("Bearer {auth_token}"));
                let builder = apply_headers(builder, headers);

                match builder.send() {
                    Ok(resp) if resp.status().is_success() => {
                        *total_deleted.lock().unwrap() += 1;
                    }
                    Ok(resp) => {
                        let status = resp.status();
                        let text = resp.text().unwrap_or_default();
                        errors
                            .lock()
                            .unwrap()
                            .push(format!("HTTP {status}: {text}"));
                    }
                    Err(e) => {
                        errors.lock().unwrap().push(format!("Request failed: {e}"));
                    }
                }
            }));
        }

        for h in handles {
            let _ = h.join();
        }
    }

    let errs = errors.lock().unwrap();
    if !errs.is_empty() {
        return Err(format!(
            "{} deletes failed, first error: {}",
            errs.len(),
            errs[0]
        ));
    }

    Ok(*total_deleted.lock().unwrap())
}

/// count documents in a collection using Firestore's aggregation query (COUNT).
/// Much cheaper than listing all documents just to measure length.
pub fn core_count_documents(
    auth_token: &str,
    project: &str,
    collection_path: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<i64, String> {
    // collection_path is relative, e.g. "users" or "users/abc/orders"
    let collection_id = collection_path
        .trim_matches('/')
        .rsplit('/')
        .next()
        .ok_or_else(|| "Empty collection path".to_string())?;

    let parent_path = {
        let trimmed = collection_path.trim_matches('/');
        match trimmed.rfind('/') {
            Some(idx) => &trimmed[..idx],
            None => "",
        }
    };

    let parent_url = format!(
        "https://firestore.googleapis.com/v1/projects/{project}/databases/(default)/documents/{parent}",
        parent = parent_path
    );
    let url = format!("{}:runAggregationQuery", parent_url.trim_end_matches('/'));

    let body = serde_json::json!({
        "structuredAggregationQuery": {
            "structuredQuery": {
                "from": [{ "collectionId": collection_id }]
            },
            "aggregations": [{
                "alias": "count",
                "count": {}
            }]
        }
    });

    let builder = client()
        .post(&url)
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

    // response is a JSON array of results; the count aggregation lands in
    // result.aggregateFields.count.integerValue
    let parsed: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("Bad aggregation response: {e}"))?;

    let count_str = parsed
        .get(0)
        .and_then(|first| first.get("result"))
        .and_then(|r| r.get("aggregateFields"))
        .and_then(|af| af.get("count"))
        .and_then(|c| c.get("integerValue"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Unexpected aggregation response shape: {text}"))?;

    count_str
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse count: {e}"))
}

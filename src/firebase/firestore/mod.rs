use crate::firebase::firestore::{
    requests::{
        core_count_documents, core_delete_collection, core_delete_collection_concurrent,
        core_delete_document, core_delete_field, core_field_increment, core_get,
        core_get_collection, core_get_document_ids, core_get_documents, core_patch,
        core_server_timestamp,
    },
    types::FirestoreResponse,
};
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::collections::HashMap;
pub mod requests;
pub mod types;

#[gen_stub_pyclass]
#[pyclass]
pub struct Document {
    auth_token: String,
    project: String,
    path: String,
    #[pyo3(get)]
    raw: Option<String>,
}

#[gen_stub_pyclass]
#[pyclass]
pub struct Field {
    auth_token: String,
    project: String,
    path: String,
    field_name: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl Field {
    #[new]
    pub fn new(auth_token: &str, project: &str, path: &str, field_name: &str) -> Self {
        Self {
            auth_token: auth_token.to_string(),
            project: project.to_string(),
            path: path.to_string(),
            field_name: field_name.to_string(),
        }
    }

    #[pyo3(name = "serverTimestamp", signature = (headers = None))]
    fn server_timestamp(
        &self,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<FirestoreResponse> {
        core_server_timestamp(
            &self.auth_token,
            &self.project,
            &self.path,
            &self.field_name,
            headers,
        )
        .map(|raw| FirestoreResponse { raw })
        .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(signature = (amount, headers = None))]
    fn increment(
        &self,
        amount: f64,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<FirestoreResponse> {
        core_field_increment(
            &self.auth_token,
            &self.project,
            &self.path,
            &self.field_name,
            amount,
            headers,
        )
        .map(|raw| FirestoreResponse { raw })
        .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(signature = (headers = None))]
    fn delete(&self, headers: Option<HashMap<String, String>>) -> PyResult<FirestoreResponse> {
        core_delete_field(
            &self.auth_token,
            &self.project,
            &self.path,
            &self.field_name,
            headers,
        )
        .map(|raw| FirestoreResponse { raw })
        .map_err(PyErr::new::<PyRuntimeError, _>)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Document {
    #[new]
    pub fn new(auth_token: String, project: String, path: String) -> Self {
        Self {
            auth_token,
            project,
            path,
            raw: None,
        }
    }

    #[pyo3(signature = (headers = None))]
    fn get(&self, headers: Option<HashMap<String, String>>) -> PyResult<FirestoreResponse> {
        core_get(&self.auth_token, &self.project, &self.path, headers)
            .map(|raw| FirestoreResponse { raw })
            .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(signature = (field_type, field_path, field_value, headers = None))]
    fn patch(
        &self,
        field_type: &str,
        field_path: &str,
        field_value: &str,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<FirestoreResponse> {
        core_patch(
            &self.auth_token,
            &self.project,
            &self.path,
            field_type,
            field_path,
            field_value,
            headers,
        )
        .map(|raw| FirestoreResponse { raw })
        .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(name = "Field")]
    fn field(&self, field_name: String) -> Field {
        Field {
            auth_token: self.auth_token.to_string(),
            project: self.project.to_string(),
            path: self.path.to_string(),
            field_name,
        }
    }

    #[pyo3(signature = (headers = None))]
    fn delete(&self, headers: Option<HashMap<String, String>>) -> PyResult<FirestoreResponse> {
        core_delete_document(&self.auth_token, &self.project, &self.path, headers)
            .map(|raw| FirestoreResponse { raw })
            .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(name = "collection")]
    fn collection(&self, collection_id: String) -> Collection {
        Collection {
            auth_token: self.auth_token.to_string(),
            project: self.project.to_string(),
            path: format!("{}/{}", self.path.trim_end_matches('/'), collection_id),
        }
    }
}

#[gen_stub_pyclass]
#[pyclass]
pub struct FirestoreClient {
    pub auth_token: String,
    pub project: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl FirestoreClient {
    #[new]
    pub fn new(auth_token: String, project: String) -> Self {
        Self {
            auth_token: auth_token.to_string(),
            project: project.to_string(),
        }
    }

    fn collection(&self, path: String) -> Collection {
        Collection {
            auth_token: self.auth_token.to_string(),
            project: self.project.to_string(),
            path,
        }
    }

    fn doc(&self, path: String) -> Document {
        Document {
            auth_token: self.auth_token.to_string(),
            project: self.project.to_string(),
            path,
            raw: None,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass]
pub struct Collection {
    auth_token: String,
    project: String,
    path: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl Collection {
    #[new]
    pub fn new(auth_token: String, project: String, path: String) -> Self {
        Self {
            auth_token,
            project,
            path,
        }
    }

    /// fetch all documents in this collection as a JSON array string
    #[pyo3(signature = (page_size = 300, headers = None))]
    fn get(
        &self,
        page_size: i32,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<FirestoreResponse> {
        core_get_collection(
            &self.auth_token,
            &self.project,
            &self.path,
            page_size,
            headers,
        )
        .map(|raw| FirestoreResponse { raw })
        .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(name = "doc")]
    fn doc(&self, doc_id: String) -> Document {
        let path = format!("{}/{}", self.path.trim_end_matches('/'), doc_id);
        Document {
            auth_token: self.auth_token.to_string(),
            project: self.project.to_string(),
            path,
            raw: None,
        }
    }

    #[pyo3(name = "getDocumentIds", signature = (headers = None))]
    fn get_document_ids(&self, headers: Option<HashMap<String, String>>) -> PyResult<Vec<String>> {
        core_get_document_ids(&self.auth_token, &self.project, &self.path, headers)
            .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    /// delete every document in this collection. Returns count deleted.
    #[pyo3(name = "deleteAll", signature = (headers = None))]
    fn delete_all(&self, headers: Option<HashMap<String, String>>) -> PyResult<u32> {
        core_delete_collection(&self.auth_token, &self.project, &self.path, headers)
            .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(name = "getDocuments", signature = (headers = None))]
    fn get_documents(&self, headers: Option<HashMap<String, String>>) -> PyResult<Vec<Document>> {
        let raws = core_get_documents(&self.auth_token, &self.project, &self.path, headers)
            .map_err(PyErr::new::<PyRuntimeError, _>)?;

        raws.into_iter()
            .map(|raw| {
                let parsed: serde_json::Value = serde_json::from_str(&raw)
                    .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("Bad doc JSON: {e}")))?;
                let name = parsed.get("name").and_then(|n| n.as_str()).ok_or_else(|| {
                    PyErr::new::<PyRuntimeError, _>("Missing document name".to_string())
                })?;
                let path = name
                    .split_once("/documents/")
                    .map(|x| x.1)
                    .ok_or_else(|| {
                        PyErr::new::<PyRuntimeError, _>("Malformed document name".to_string())
                    })?
                    .to_string();

                Ok(Document {
                    auth_token: self.auth_token.clone(),
                    project: self.project.clone(),
                    path,
                    raw: Some(raw),
                })
            })
            .collect()
    }

    #[pyo3(name = "deleteAllConcurrent", signature = (concurrency = 20, headers = None))]
    fn delete_all_concurrent(
        &self,
        concurrency: usize,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<u32> {
        core_delete_collection_concurrent(
            &self.auth_token,
            &self.project,
            &self.path,
            concurrency,
            headers,
        )
        .map_err(PyErr::new::<PyRuntimeError, _>)
    }

    #[pyo3(name = "documentCount", signature = (headers = None))]
    fn count(&self, headers: Option<HashMap<String, String>>) -> PyResult<i64> {
        core_count_documents(&self.auth_token, &self.project, &self.path, headers)
            .map_err(PyErr::new::<PyRuntimeError, _>)
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FirestoreClient>()?;
    m.add_class::<Collection>()?;
    m.add_class::<Document>()?;
    m.add_class::<Field>()?;
    m.add_class::<FirestoreResponse>()?;
    Ok(())
}

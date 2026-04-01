use crate::firebase::firestore::{
    requests::{core_get, core_patch},
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

    fn doc(&self, path: String) -> Document {
        Document {
            auth_token: self.auth_token.to_string(),
            project: self.project.to_string(),
            path,
        }
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FirestoreClient>()?;
    m.add_class::<Document>()?;
    m.add_class::<FirestoreResponse>()?;
    Ok(())
}

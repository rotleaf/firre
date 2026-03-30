use std::collections::HashMap;

use pyo3::{exceptions::PyRuntimeError, prelude::*};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde_json::Value;

pub mod requests;
pub mod types;

use crate::firebase::auth::{
    requests::core_refresh_token,
    types::{AuthResponse, RefreshResponse},
};

#[pyclass]
#[gen_stub_pyclass]
pub struct Auth {
    pub api_key: String,
}

#[allow(non_snake_case)]
#[gen_stub_pymethods]
#[pymethods]
impl Auth {
    #[pyo3(text_signature = "(refresh_token, headers=None)")]
    #[pyo3(signature = (refresh_token, headers=None))]
    fn refreshTokenAuth(
        &self,
        refresh_token: &str,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<RefreshResponse> {
        let raw = core_refresh_token(&self.api_key, refresh_token, headers)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;

        let json: Value = serde_json::from_str(&raw)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

        Ok(RefreshResponse {
            id_token: json["id_token"].as_str().unwrap_or("").to_string(),
            access_token: json["access_token"].as_str().unwrap_or("").to_string(),
            refresh_token: json["refresh_token"].as_str().unwrap_or("").to_string(),
            user_id: json["user_id"].as_str().unwrap_or("").to_string(),
            expires_in: json["expires_in"].as_str().unwrap_or("").to_string(),
            raw,
        })
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Auth>()?;
    m.add_class::<AuthResponse>()?;
    m.add_class::<RefreshResponse>()?;
    Ok(())
}

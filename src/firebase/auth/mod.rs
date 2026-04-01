use std::collections::HashMap;

use pyo3::{exceptions::PyRuntimeError, prelude::*};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde_json::Value;

pub mod requests;
pub mod types;

use crate::firebase::auth::{
    requests::{core_email_pwd_sign_in, core_email_pwd_sign_up, core_refresh_token},
    types::{AuthResponse, RefreshResponse},
};

#[pyclass]
#[gen_stub_pyclass]
pub struct Auth {
    pub api_key: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl Auth {
    #[pyo3(text_signature = "(email, password, headers=None)")]
    #[pyo3(name="emailPwdSignIn",signature = (email, password, headers=None))]
    fn email_pwd_sign_in(
        &self,
        email: &str,
        password: &str,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<AuthResponse> {
        let raw = core_email_pwd_sign_in(&self.api_key, email, password, headers)
            .map_err(PyErr::new::<PyRuntimeError, _>)?;

        let json: Value = serde_json::from_str(&raw)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

        Ok(AuthResponse {
            auth_header: String::new(),
            id_token: json["idToken"].as_str().unwrap_or("").to_string(),
            refresh_token: json["refreshToken"].as_str().unwrap_or("").to_string(),
            email: json["email"].as_str().unwrap_or("").to_string(),
            user_id: json["localId"].as_str().unwrap_or("").to_string(),
            expires_in: json["expiresIn"].as_str().unwrap_or("").to_string(),
            raw,
        })
    }

    #[pyo3(name="emailPwdSignUp",signature = (email, password, headers=None))]
    fn email_pwd_sign_up(
        &self,
        email: &str,
        password: &str,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<AuthResponse> {
        let raw = core_email_pwd_sign_up(&self.api_key, email, password, headers)
            .map_err(PyErr::new::<PyRuntimeError, _>)?;
        let json: serde_json::Value = serde_json::from_str(&raw)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
        Ok(AuthResponse {
            auth_header: String::new(),
            id_token: json["idToken"].as_str().unwrap_or("").to_string(),
            refresh_token: json["refreshToken"].as_str().unwrap_or("").to_string(),
            email: json["email"].as_str().unwrap_or("").to_string(),
            user_id: json["localId"].as_str().unwrap_or("").to_string(),
            expires_in: json["expiresIn"].as_str().unwrap_or("").to_string(),
            raw,
        })
    }

    #[pyo3(text_signature = "(refresh_token, headers=None)")]
    #[pyo3(name="refreshTokenAuth",signature = (refresh_token, headers=None))]
    fn refresh_token_auth(
        &self,
        refresh_token: &str,
        headers: Option<HashMap<String, String>>,
    ) -> PyResult<RefreshResponse> {
        let raw = core_refresh_token(&self.api_key, refresh_token, headers)
            .map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)?;

        let json: Value = serde_json::from_str(&raw)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

        Ok(RefreshResponse {
            auth_header: String::new(),
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

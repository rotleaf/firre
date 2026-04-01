use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::collections::HashMap;

#[gen_stub_pyclass]
#[pyclass]
pub struct AuthResponse {
    pub id_token: String,
    pub refresh_token: String,
    pub email: String,
    pub user_id: String,
    pub expires_in: String,
    pub auth_header: String,
    pub raw: String,
}

#[gen_stub_pyclass]
#[pyclass]
pub struct RefreshResponse {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub expires_in: String,
    pub auth_header: String,
    pub raw: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl RefreshResponse {
    #[getter]
    #[pyo3(name = "getAuthHeader")]
    fn get_auth_header(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.id_token),
        );
        map
    }

    #[getter]
    fn json<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        let json_mod = PyModule::import(py, "json")?;
        let obj = json_mod.getattr("loads")?.call1((self.raw.as_str(),))?;
        Ok(obj.unbind())
    }

    #[getter]
    #[pyo3(name = "idToken")]
    fn id_token(&self) -> &str {
        &self.id_token
    }

    #[getter]
    #[pyo3(name = "refreshToken")]
    fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
    #[getter]
    #[pyo3(name = "accessToken")]
    fn access_token(&self) -> &str {
        &self.access_token
    }
    #[getter]
    #[pyo3(name = "userId")]
    fn user_id(&self) -> &str {
        &self.user_id
    }
    #[getter]
    #[pyo3(name = "expiresIn")]
    fn expires_in(&self) -> &str {
        &self.expires_in
    }
    #[getter]
    fn raw(&self) -> &str {
        &self.raw
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl AuthResponse {
    #[getter]
    #[pyo3(name = "getAuthHeader")]
    fn get_auth_header(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.id_token),
        );
        map
    }

    #[getter]
    fn json<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        let json_mod = PyModule::import(py, "json")?;
        let obj = json_mod.getattr("loads")?.call1((self.raw.as_str(),))?;
        Ok(obj.unbind())
    }
    #[getter]
    #[pyo3(name = "idToken")]
    fn id_token(&self) -> &str {
        &self.id_token
    }
    #[getter]
    #[pyo3(name = "refreshToken")]
    fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
    #[getter]
    fn email(&self) -> &str {
        &self.email
    }
    #[getter]
    #[pyo3(name = "userId")]
    fn user_id(&self) -> &str {
        &self.user_id
    }
    #[getter]
    #[pyo3(name = "expiresIn")]
    fn expires_in(&self) -> &str {
        &self.expires_in
    }
    #[getter]
    fn raw(&self) -> &str {
        &self.raw
    }
}

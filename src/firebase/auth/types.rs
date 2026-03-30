use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass]
pub struct AuthResponse {
    pub id_token: String,
    pub refresh_token: String,
    pub email: String,
    pub user_id: String,
    pub expires_in: String,
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
    pub raw: String,
}

#[pymethods]
#[gen_stub_pymethods]
#[allow(non_snake_case)]
impl RefreshResponse {
    #[getter]
    fn idToken(&self) -> &str {
        &self.id_token
    }
    #[getter]
    fn refreshToken(&self) -> &str {
        &self.refresh_token
    }
    #[getter]
    fn accessToken(&self) -> &str {
        &self.refresh_token
    }
    #[getter]
    fn userId(&self) -> &str {
        &self.user_id
    }
    #[getter]
    fn expiresIn(&self) -> &str {
        &self.expires_in
    }
    #[getter]
    fn raw(&self) -> &str {
        &self.raw
    }
}

#[pymethods]
#[gen_stub_pymethods]
#[allow(non_snake_case)]
impl AuthResponse {
    #[getter]
    fn idToken(&self) -> &str {
        &self.id_token
    }
    #[getter]
    fn refreshToken(&self) -> &str {
        &self.refresh_token
    }
    #[getter]
    fn email(&self) -> &str {
        &self.email
    }
    #[getter]
    fn userId(&self) -> &str {
        &self.user_id
    }
    #[getter]
    fn expiresIn(&self) -> &str {
        &self.expires_in
    }
    #[getter]
    fn raw(&self) -> &str {
        &self.raw
    }
}

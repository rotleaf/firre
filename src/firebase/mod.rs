use pyo3::prelude::*;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde_json::Value;

use crate::firebase::auth::Auth;

pub mod auth;

#[gen_stub_pyclass]
#[pyclass]
pub struct Firebase {
    api_key: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl Firebase {
    #[new]
    pub fn new(api_key: String) -> Self {
        Firebase {
            api_key: api_key.to_string(),
        }
    }

    #[getter]
    fn auth(&self) -> Auth {
        Auth {
            api_key: self.api_key.clone(),
        }
    }
}

fn firebase_error(json: &Value) -> Result<(), String> {
    if let Some(error) = json.get("error") {
        let message = error["message"]
            .as_str()
            .unwrap_or("Unknown Firebase error");
        return Err(message.into());
    }
    Ok(())
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Firebase>()?;
    auth::register(m)?;
    Ok(())
}

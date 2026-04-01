use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass]
pub struct FirestoreResponse {
    pub raw: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl FirestoreResponse {
    #[getter]
    fn raw(&self) -> &str {
        &self.raw
    }

    #[getter]
    fn json<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        let json_mod = PyModule::import(py, "json")?;
        let obj = json_mod.getattr("loads")?.call1((self.raw.as_str(),))?;
        Ok(obj.unbind())
    }
}

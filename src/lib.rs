use pyo3::prelude::*;

pub mod firebase;
pub mod utils;

pyo3_stub_gen::define_stub_info_gatherer!(stub_info);

#[pymodule]
pub fn firre(m: &Bound<'_, PyModule>) -> PyResult<()> {
    firebase::register(m)?;
    Ok(())
}

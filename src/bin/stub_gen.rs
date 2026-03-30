use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = firre::stub_info()?;
    stub.generate()?;
    Ok(())
}

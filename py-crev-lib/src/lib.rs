use pyo3;
use pyo3::prelude::*;

mod config;
mod edit;
mod id;
mod prelude;

#[pymodule]
fn py_crev_lib(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    crate::id::add_submodule(module)?;
    crate::config::add_submodule(module)?;
    Ok(())
}

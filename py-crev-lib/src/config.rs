use crev_lib;

use pyo3;
use pyo3::prelude::*;

use crate::prelude::*;

#[pyfunction]
fn edit() -> PyResult<()> {
    || -> Result<()> {
        let local = crev_lib::Local::auto_create_or_open()?;
        crate::edit::edit_user_config(&local)?;
        Ok(())
    }()
    .map_err(From::from)
}

#[pymodule]
fn config(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pyfunction!(edit))?;
    Ok(())
}

pub fn add_submodule(module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pymodule!(config))?;
    Ok(())
}

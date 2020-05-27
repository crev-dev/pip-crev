use crev_lib;

use pyo3;
use pyo3::prelude::*;

use self::prelude::*;
mod prelude;

#[pyfunction]
fn show_current_user_public_ids() -> PyResult<()> {
    || -> Result<()> {
        let local = crev_lib::Local::auto_open()?;
        local.show_current_user_public_ids()?;
        Ok(())
    }()
    .map_err(From::from)
}

#[pyfunction]
fn switch_id(id: String) -> PyResult<()> {
    || -> Result<()> {
        let local = crev_lib::Local::auto_open()?;
        local.switch_id(&id)?;
        Ok(())
    }()
    .map_err(From::from)
}

#[pyfunction]
fn export_id(id: Option<String>) -> PyResult<String> {
    || -> Result<String> {
        let local = crev_lib::Local::auto_open()?;
        local.export_locked_id(id).map_err(From::from)
    }()
    .map_err(From::from)
}

#[pymodule]
fn py_crev_lib(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pyfunction!(show_current_user_public_ids))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(switch_id))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(export_id))?;
    Ok(())
}

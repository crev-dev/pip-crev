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
    }().map_err(From::from)
}

#[pymodule]
fn py_crev_lib(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pyfunction!(show_current_user_public_ids))?;
    Ok(())
}

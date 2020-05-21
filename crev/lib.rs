// Source adopted from
// https://github.com/tildeio/helix-website/blob/master/crates/crev/src/lib.rs

use crev_lib;

use pyo3;
use pyo3::prelude::*;

#[pyfunction]
fn show_current_user_public_ids() {
    let local = crev_lib::Local::auto_open().unwrap();
    local.show_current_user_public_ids().unwrap();
}

#[pymodule]
fn py_crev_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(pyo3::wrap_pyfunction!(show_current_user_public_ids))?;
    Ok(())
}

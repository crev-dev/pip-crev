use crev_lib;

use pyo3;
use pyo3::prelude::*;

use std::io::BufRead;

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

fn load_stdin_with_prompt() -> Result<Vec<u8>> {
    println!("Paste in the text and press Ctrl+D.");
    let mut s = vec![];
    std::io::stdin().lock().read_until(0, &mut s)?;
    Ok(s)
}

#[pyfunction]
fn import_id() -> PyResult<()> {
    || -> Result<()> {
        let local = crev_lib::Local::auto_create_or_open()?;
        let s = load_stdin_with_prompt()?;
        let id = local.import_locked_id(&String::from_utf8(s)?)?;
        local.save_current_id(&id.id)?;
        let url = &id
            .url
            .as_ref()
            .expect("A public id must have an associated URL");
        let proof_dir_path = local.get_proofs_dir_path_for_url(url)?;
        if !proof_dir_path.exists() {
            local.clone_proof_dir_from_git(&url.url, false)?;
        }
        Ok(())
    }()
    .map_err(From::from)
}

#[pymodule]
fn py_crev_lib(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pyfunction!(show_current_user_public_ids))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(switch_id))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(export_id))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(import_id))?;
    Ok(())
}

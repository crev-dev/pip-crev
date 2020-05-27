use crev_common;
use crev_lib;

use pyo3;
use pyo3::prelude::*;

use std::io::{self, BufRead};

use self::prelude::*;
mod prelude;

#[pyfunction]
fn new_id(
    url: Option<String>,
    github_username: Option<String>,
    use_https_push: bool,
) -> PyResult<()> {
    || -> Result<()> {
        let url = match (url, github_username) {
            (Some(url), None) => url,
            (None, Some(username)) => format!("https://github.com/{}/crev-proofs", username),
            _ => {
                return Err(PyCrevLibError::PyCrevLib(
                    "Must provide either a github username or url, but not both.".to_string(),
                ))
            }
        };
        if !url.starts_with("https://") {
            return Err(PyCrevLibError::PyCrevLib(
                "URL must start with 'https://'".to_string(),
            ));
        }

        fn read_new_passphrase() -> io::Result<String> {
            println!("CrevID will be protected by a passphrase.");
            println!("There's no way to recover your CrevID if you forget your passphrase.");
            crev_common::read_new_passphrase()
        }
        let local = crev_lib::Local::auto_create_or_open()?;
        let locked_id = local
            .generate_id(&url, use_https_push, read_new_passphrase)
            .map_err(|e| {
                eprintln!("To create your proof repository, fork the template:");
                eprintln!("https://github.com/crev-dev/crev-proofs/fork");
                eprintln!("For help visit: https://github.com/crev-dev/crev/wiki/Proof-Repository");
                eprintln!();
                e
            })?;
        println!("Your CrevID was created and will be printed below in an encrypted form.");
        println!("Make sure to back it up on another device, to prevent losing it.");
        println!("{}", locked_id);
        Ok(())
    }()
    .map_err(From::from)
}

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
    module.add_wrapped(pyo3::wrap_pyfunction!(new_id))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(show_current_user_public_ids))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(switch_id))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(export_id))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(import_id))?;
    Ok(())
}

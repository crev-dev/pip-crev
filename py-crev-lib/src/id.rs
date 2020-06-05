use crev_common;
use crev_data::{self, proof::content::ContentExt};
use crev_lib;

use pyo3;
use pyo3::prelude::*;
use std::io::{self, BufRead};

use crate::prelude::*;

#[pyfunction]
fn new(url: Option<String>, github_username: Option<String>, use_https_push: bool) -> PyResult<()> {
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
fn get_current() -> PyResult<()> {
    || -> Result<()> {
        let local = crev_lib::Local::auto_open()?;
        let current = local
            .read_current_locked_id_opt()?
            .map(|locked_id| locked_id.to_public_id());
        for public_id in local.get_current_user_public_ids()? {
            let is_current = current.as_ref().map_or(false, |c| c.id == public_id.id);
            println!(
                "{} {}{}",
                public_id.id,
                public_id.url_display(),
                if is_current { " (current)" } else { "" }
            );
        }
        Ok(())
    }()
    .map_err(From::from)
}

#[pyfunction]
fn switch(id: String) -> PyResult<()> {
    || -> Result<()> {
        let local = crev_lib::Local::auto_open()?;
        local.switch_id(&id)?;
        Ok(())
    }()
    .map_err(From::from)
}

#[pyfunction]
fn export(id: Option<String>) -> PyResult<String> {
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
pub fn import_id() -> PyResult<()> {
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

/// Creates a new trust proof interactively
pub fn create_id_trust_proof_interactively(
    ids: &[crev_data::Id],
    trust_or_distrust: crev_lib::TrustProofType,
) -> Result<crev_data::proof::Proof> {
    let local = crev_lib::Local::auto_open()?;
    let unlocked_id = local.read_current_unlocked_id(&crev_common::read_passphrase)?;
    let trust = crate::edit::build_trust_proof_interactively(
        &local,
        unlocked_id.as_public_id(),
        ids.to_vec(),
        trust_or_distrust,
    )?;
    Ok(trust.sign_by(&unlocked_id)?)
}

fn ids_from_string(id_strings: &Vec<String>) -> Result<Vec<crev_data::Id>> {
    id_strings
        .iter()
        .map(|s| match crev_data::Id::crevid_from_str(&s) {
            Ok(s) => Ok(s),
            Err(e) => {
                return Err(PyCrevLibError::PyCrevLib(format!(
                    "'{}' is not a valid crev Id: {}",
                    s, e
                )))
            }
        })
        .collect()
}

/// Given an ID trust proof type as a Python String, returns the corresponding crev_lib type.
fn get_proof_type_from_string(
    proof_type: &pyo3::types::PyString,
) -> PyResult<crev_lib::TrustProofType> {
    let proof_type = proof_type.to_string()?.to_string();
    Ok(match proof_type.as_str() {
        "Trust" => crev_lib::TrustProofType::Trust,
        "Untrust" => crev_lib::TrustProofType::Untrust,
        "Distrust" => crev_lib::TrustProofType::Distrust,
        _ => {
            return Err(pyo3::PyErr::new::<pyo3::exceptions::TypeError, _>(format!(
                "Unknown trust type: {}",
                proof_type
            )))
        }
    })
}

#[pyfunction]
pub fn create_proof(
    ids: &pyo3::types::PyList,
    proof_type: &pyo3::types::PyString,
    no_commit: &pyo3::types::PyBool,
    print_unsigned: &pyo3::types::PyBool,
    print_signed: &pyo3::types::PyBool,
    no_store: &pyo3::types::PyBool,
) -> PyResult<()> {
    let ids: Vec<String> = ids.iter().map(|x| x.to_string()).collect();
    let ids = ids_from_string(&ids)?;
    let proof_type = get_proof_type_from_string(&proof_type)?;

    || -> Result<()> {
        let proof = create_id_trust_proof_interactively(&ids, proof_type)?;

        if print_unsigned.is_true() {
            print!("{}", proof.body());
        }
        if print_signed.is_true() {
            print!("{}", proof);
        }
        if !no_store.is_true() {
            crev_lib::proof::store_id_trust_proof(&proof, &ids, proof_type, !no_commit.is_true())?;
        }
        Ok(())
    }()
    .map_err(From::from)
}

#[pymodule]
fn id(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pyfunction!(new))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(get_current))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(switch))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(export))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(import_id))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(create_proof))?;
    Ok(())
}

pub fn add_submodule(module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pymodule!(id))?;
    Ok(())
}

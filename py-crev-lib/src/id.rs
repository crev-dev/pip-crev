use crev_common;
use crev_data::{self, proof::content::ContentExt};
use crev_lib::{self, ProofStore};

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

fn maybe_store(
    local: &crev_lib::Local,
    proof: &crev_data::proof::Proof,
    commit_msg: &str,
    no_commit: bool,
    print_unsigned: bool,
    print_signed: bool,
    no_store: bool,
) -> Result<()> {
    if print_unsigned {
        print!("{}", proof.body());
    }

    if print_signed {
        print!("{}", proof);
    }

    if !no_store {
        local.insert(&proof)?;

        if !no_commit {
            local.proof_dir_commit(&commit_msg)?;
        }
    }
    Ok(())
}

/// Creates new trust proof interactively
pub fn create_trust_proof(
    ids: Vec<crev_data::Id>,
    trust_or_distrust: crev_lib::TrustProofType,
    no_commit: bool,
    print_unsigned: bool,
    print_signed: bool,
    no_store: bool,
) -> Result<()> {
    let local = crev_lib::Local::auto_open()?;
    let unlocked_id = local.read_current_unlocked_id(&crev_common::read_passphrase)?;

    let string_ids = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let trust = crate::edit::build_trust_proof_interactively(
        &local,
        unlocked_id.as_public_id(),
        ids,
        trust_or_distrust,
    )?;

    let proof = trust.sign_by(&unlocked_id)?;
    let commit_msg = format!(
        "Add {t_or_d} for {ids}",
        t_or_d = trust_or_distrust,
        ids = string_ids
    );

    maybe_store(
        &local,
        &proof,
        &commit_msg,
        no_commit,
        print_unsigned,
        print_signed,
        no_store,
    )?;
    Ok(())
}

fn ids_from_string(id_strings: &Vec<&str>) -> Result<Vec<crev_data::Id>> {
    id_strings
        .iter()
        .map(|s| match crev_data::Id::crevid_from_str(&s) {
            Ok(s) => Ok(s),
            Err(e) => {
                return Err(PyCrevLibError::PyCrevLib(format!(
                    "'{}' is not a valid crev Id: {}",
                    s, e
                )))
            } //bail!("'{}' is not a valid crev Id: {}", s, e),
        })
        .collect()
}

#[pyfunction]
pub fn wrap_create_trust_proof(
    id: &str,                //Vec<crev_data::Id>,
    trust_or_distrust: &str, //crev_lib::TrustProofType,
    no_commit: bool,
    print_unsigned: bool,
    print_signed: bool,
    no_store: bool,
) -> PyResult<()> {
    let ids = vec![id];
    let ids = ids_from_string(&ids)?;
    let trust_or_distrust = match trust_or_distrust {
        "Trust" => crev_lib::TrustProofType::Trust,
        "Untrust" => crev_lib::TrustProofType::Untrust,
        "Distrust" => crev_lib::TrustProofType::Distrust,
        _ => crev_lib::TrustProofType::Trust, //return Err(pyo3::PyErr("Unknown trust type.".to_string())),
    };

    || -> Result<()> {
        create_trust_proof(
            ids,
            trust_or_distrust,
            no_commit,
            print_unsigned,
            print_signed,
            no_store,
        )
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
    module.add_wrapped(pyo3::wrap_pyfunction!(wrap_create_trust_proof))?;
    Ok(())
}

pub fn add_submodule(module: &PyModule) -> PyResult<()> {
    module.add_wrapped(pyo3::wrap_pymodule!(id))?;
    Ok(())
}

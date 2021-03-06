use anyhow;
use pyo3::prelude::*;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum PyCrevLibError {
    #[error("ERROR: {}", _0)]
    PyCrevLib(String),

    #[error("crev-lib error: {}", _0)]
    CrevLib(String),

    #[error("UTF error: {}", _0)]
    UtfError(String),

    #[error("IO error: {}", _0)]
    IoError(String),
}

impl std::convert::From<PyCrevLibError> for PyErr {
    fn from(err: PyCrevLibError) -> PyErr {
        pyo3::exceptions::OSError::py_err(err.to_string())
    }
}

impl std::convert::From<crev_lib::Error> for PyCrevLibError {
    fn from(err: crev_lib::Error) -> PyCrevLibError {
        PyCrevLibError::CrevLib(err.to_string())
    }
}

impl std::convert::From<&str> for PyCrevLibError {
    fn from(err: &str) -> PyCrevLibError {
        PyCrevLibError::PyCrevLib(err.to_string())
    }
}

impl std::convert::From<std::io::Error> for PyCrevLibError {
    fn from(err: std::io::Error) -> PyCrevLibError {
        PyCrevLibError::IoError(err.to_string())
    }
}

impl std::convert::From<anyhow::Error> for PyCrevLibError {
    fn from(err: anyhow::Error) -> PyCrevLibError {
        PyCrevLibError::IoError(err.to_string())
    }
}

impl std::convert::From<crev_data::Error> for PyCrevLibError {
    fn from(err: crev_data::Error) -> PyCrevLibError {
        PyCrevLibError::IoError(err.to_string())
    }
}

impl std::convert::From<std::string::FromUtf8Error> for PyCrevLibError {
    fn from(err: std::string::FromUtf8Error) -> PyCrevLibError {
        PyCrevLibError::UtfError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, PyCrevLibError>;

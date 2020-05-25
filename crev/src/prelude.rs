use thiserror;
use pyo3::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum PyCrevLibError {
    #[error("crev-lib error: {}", _0)]
    CrevLib(String),
}

impl std::convert::From<crev_lib::Error> for PyCrevLibError {
    fn from(err: crev_lib::Error) -> PyCrevLibError {
        PyCrevLibError::CrevLib(err.to_string())
    }
}

impl std::convert::From<PyCrevLibError> for PyErr {
    fn from(err: PyCrevLibError) -> PyErr {
        pyo3::exceptions::OSError::py_err(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, PyCrevLibError>;
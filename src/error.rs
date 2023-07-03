use std::{error, fmt};

use crate::ErrorData;
use reqwest::StatusCode;

/// A common error type returned from Nano API operations
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Tried to login but in guest mode
    NoCredentials,
    /// Valid JSON but can't parse it
    BadJSON(serde_json::Value),
    /// An error induced by a failed reqwest
    ReqwestError(reqwest::Error),
    /// An error caused by an invalid response from the Nano API
    SimpleNanoError(StatusCode, String),
    /// An error from Nano with multiple complex inner values
    NanoErrors(Vec<ErrorData>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoCredentials => write!(f, "No credentials available"),
            Error::BadJSON(val) => write!(f, "Valid JSON doesn't parse: {val}"),
            Error::ReqwestError(err) => write!(f, "Reqwest Error: {err}"),
            Error::SimpleNanoError(code, message) => write!(
                f,
                "NanoWrimo API Error: {message} (status code {})",
                code.as_u16()
            ),
            Error::NanoErrors(errs) => errs
                .iter()
                .map(|err| {
                    write!(
                        f,
                        "{} ({}): {} (status code {})",
                        err.title, err.code, err.detail, err.status
                    )
                })
                .collect::<Result<_, _>>(),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ReqwestError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

use crate::response::ErrorResponse;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NoLoginToken,
    ApiError(ErrorResponse),
    HTTPInternalError(String),
    DeserealizationError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoLoginToken => write!(f, "Not logged in or no valid token supplied"),
            Error::ApiError(resp) => write!(f, "Error response from Sigma: {:?}", resp),
            Error::HTTPInternalError(resp) => write!(f, "HTTP Request failed: {:}", resp),
            Error::DeserealizationError(resp) => {
                write!(f, "Could not deserialize response: {}", resp)
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPInternalError(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::DeserealizationError(e.to_string())
    }
}

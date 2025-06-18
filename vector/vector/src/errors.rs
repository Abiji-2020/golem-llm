use crate::golem::vector::types::VectorError;
use reqwest::StatusCode;

pub fn unsupported(what: impl AsRef<str>) -> VectorError {
    VectorError::UnsupportedFeature(format!("Unsupported: {}", what.as_ref()))
}

pub fn from_reqwest_error(details: impl AsRef<str>, err: reqwest::Error) -> VectorError {
    VectorError::ConnectionError(format!("{}: {err}", details.as_ref()))
}

pub fn error_code_from_status(status: StatusCode) -> VectorError {
    if status == StatusCode::TOO_MANY_REQUESTS {
        VectorError::RateLimited("Too many requests".to_string())
    } else if status == StatusCode::UNAUTHORIZED
        || status == StatusCode::FORBIDDEN
        || status == StatusCode::PAYMENT_REQUIRED
    {
        VectorError::Unauthorized("Authentication failed".to_string())
    } else if status.is_client_error() {
        VectorError::ConnectionError(format!("Client error: {}", status))
    } else {
        VectorError::ProviderError(format!("Internal error: {}", status))
    }
}

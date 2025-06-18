use crate::golem::vector::types::VectorError;
use std::ffi::OsStr;

pub fn with_config_key<R>(
    key: impl AsRef<OsStr>,
    fail: impl FnOnce(VectorError) -> R,
    succeed: impl FnOnce(String) -> R,
) -> R {
    let key_str = key.as_ref().to_string_lossy().to_string();
    match std::env::var(key) {
        Ok(value) => succeed(value),
        Err(_) => {
            let error =
                VectorError::ConnectionError(format!("Missing config Key: {key_str}").to_string());
            fail(error)
        }
    }
}

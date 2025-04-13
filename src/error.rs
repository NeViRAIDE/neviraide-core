use std::result::Result as StdResult;

use nvim_oxi::{Error as OxiError, api::Error as OxiApiError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NeviraideError {
    #[error("Unknown error occurred")]
    Unknown,

    #[error("Neovim API error occurred: {0}")]
    NeovimApiError(String),

    #[error("Oxi API error occurred: {0}")]
    OxiApiError(String)
}

// General Result type alias for the plugin, combining OxiResult and
// NeviraideError
pub type NeviraideResult<T> = StdResult<T, NeviraideError>;

impl From<OxiError> for NeviraideError {
    fn from(error: OxiError) -> Self {
        match error {
            OxiError::Api(api_error) => {
                NeviraideError::OxiApiError(format!("API error: {:?}", api_error))
            }
            OxiError::Nvim(nvim_error) => {
                NeviraideError::NeovimApiError(format!("Neovim error: {:?}", nvim_error))
            }
            OxiError::ObjectConversion(conversion_error) => NeviraideError::OxiApiError(format!(
                "Object conversion error: {:?}",
                conversion_error
            )),
            OxiError::Serialize(serialize_error) => {
                NeviraideError::OxiApiError(format!("Serialization error: {:?}", serialize_error))
            }
            OxiError::Deserialize(deserialize_error) => NeviraideError::OxiApiError(format!(
                "Deserialization error: {:?}",
                deserialize_error
            )),
            OxiError::Libuv(libuv_error) => {
                NeviraideError::OxiApiError(format!("Libuv error: {:?}", libuv_error))
            }
            _ => NeviraideError::OxiApiError(format!("Unknown OxiError: {:?}", error))
        }
    }
}

// Implementing conversion from `OxiApiError` to `NeviraideError`
impl From<OxiApiError> for NeviraideError {
    fn from(error: OxiApiError) -> Self {
        NeviraideError::OxiApiError(format!("{:?}", error))
    }
}

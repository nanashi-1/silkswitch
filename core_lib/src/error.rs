use base64::DecodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Encryption failed (buffer too small or data malformed)")]
    Encryption,
}

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Cipher error")]
    Cipher(#[from] CipherError),

    #[error("Base64 error")]
    Base64(#[from] DecodeError),

    #[error("Failed to parse file")]
    ParsingError,
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Serde failed to parse JSON")]
    JSON(#[from] serde_json::Error),

    #[error("Failed to get value from pointer: {0}")]
    Pointer(String),

    #[error("Wrong value type")]
    ValueType,

    #[error("Cannot parse value")]
    ValueParse,
}

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

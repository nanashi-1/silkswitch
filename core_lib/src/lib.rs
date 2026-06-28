use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{
    cipher::{decrypt, encrypt},
    error::EncodingError,
    headers::{add_header, remove_header},
};

mod cipher;
mod error;
mod headers;
mod values;

pub fn encode(bytes: &[u8]) -> Result<Vec<u8>, EncodingError> {
    let encrypted_bytes = encrypt(bytes)?;
    let base64 = BASE64_STANDARD.encode(encrypted_bytes);

    Ok(add_header(base64.as_bytes()))
}

pub fn decode(bytes: &[u8]) -> Result<Vec<u8>, EncodingError> {
    let headerless_bytes = remove_header(bytes).ok_or(EncodingError::ParsingError)?;
    let un_base64ed_bytes = BASE64_STANDARD.decode(headerless_bytes)?;

    Ok(decrypt(&un_base64ed_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversible_test() {
        let bytes = "Reversible Test".as_bytes();

        let encoded_bytes = encode(bytes).unwrap();
        let decoded_bytes = decode(&encoded_bytes).unwrap();

        assert_eq!(bytes, decoded_bytes);
    }
}

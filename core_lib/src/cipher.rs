use aes::{
    Aes256,
    cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyInit, block_padding::Pkcs7},
};
use ecb::{Decryptor, Encryptor};

use crate::error::CipherError;

const KEY: [u8; 32] = *b"UKu52ePUBwetZ9wNX88o54dnfKRu0T1l";

pub fn encrypt(bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
    let bytes_len = bytes.len();

    let mut buf = vec![0u8; bytes_len + 16];
    buf[..bytes_len].copy_from_slice(bytes);

    let encrypted_bytes_len = Encryptor::<Aes256>::new(&KEY.into())
        .encrypt_padded::<Pkcs7>(buf.as_mut_slice(), bytes.len())
        .map_err(|_| CipherError::Encryption)?
        .len();

    buf.truncate(encrypted_bytes_len);
    Ok(buf)
}

pub fn decrypt(bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
    let mut buf = bytes.to_vec();

    let decrypted_bytes_len = Decryptor::<Aes256>::new(&KEY.into())
        .decrypt_padded::<Pkcs7>(buf.as_mut_slice())
        .map_err(|_| CipherError::Encryption)?
        .len();

    buf.truncate(decrypted_bytes_len);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversible_test() {
        let original_bytes = "Reversible Test".as_bytes();

        let encrypted_bytes = encrypt(original_bytes).unwrap();
        let decrypted_bytes = decrypt(&encrypted_bytes).unwrap();

        assert_eq!(original_bytes, decrypted_bytes);
    }
}

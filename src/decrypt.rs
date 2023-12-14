use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::Aead;
use windows::{
    Win32::Security::Cryptography::CryptUnprotectData,
    Win32::Security::Cryptography::CRYPT_INTEGER_BLOB,
};
use crate::error::ExporterError;

pub fn decrypt_value(master_key: Option<Vec<u8>>, value: Vec<u8>) -> String {
    match master_key {
        None => match dpapi_crypt_unprotect_data(value) {
            Ok(buf) => String::from_utf8(buf).unwrap_or("Failed to decode utf-8".to_string()),
            Err(_) => "Failed to use DPAPI crypt unprotect data".to_string()
        }
        Some(mk) => {
            aes_gcm_256(mk.clone().as_mut_slice(), value).unwrap()
        }
    }
}

pub fn aes_gcm_256(key_buf: &mut [u8], pwd_buf: Vec<u8>) -> Result<String, ExporterError> {
    let key = GenericArray::from_slice(key_buf);
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(&pwd_buf[3..15]);

    let plaintext = cipher.decrypt(nonce, &pwd_buf[15..]).map_err(
        |_| ExporterError::IO("Failed to decrypt AES256".to_string())
    )?;

    String::from_utf8(Vec::from(plaintext))
        .map_err(|_| ExporterError::IO("TODO: FINISH THIS".to_string()))
}

pub fn dpapi_crypt_unprotect_data(mut data_buf: Vec<u8>) -> Result<Vec<u8>, ExporterError> {
    let source = CRYPT_INTEGER_BLOB {
        cbData: data_buf.len() as u32,
        pbData: data_buf.as_mut_ptr(),
    };

    let mut out_key = Vec::new();

    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    unsafe {
        CryptUnprotectData(
            &source,
            None,
            None,
            None,
            None,
            0,
            &mut output,
        ).map_err(|e| {
            ExporterError::IO("TODO: FINISH THIS".to_string())
        })?;

        out_key = Vec::from_raw_parts(
            output.pbData, output.cbData as usize, output.cbData as usize
        );
    }

    Ok(out_key)
}

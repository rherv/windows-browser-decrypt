use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::Aead;
use windows::{
    Win32::Security::Cryptography::CryptUnprotectData,
    Win32::Security::Cryptography::CRYPT_INTEGER_BLOB,
};

pub fn aes_gcm_256(key_buf: &mut [u8], pwd_buf: Vec<u8>) -> Result<String, String> {
    let key = GenericArray::from_slice(key_buf);
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(&pwd_buf[3..15]);

    let plaintext = cipher.decrypt(nonce, &pwd_buf[15..]).map_err(
        |_| "Failed to decrypt AES256".to_string()
    )?;

    String::from_utf8(Vec::from(plaintext))
        .map_err(|_| "Failed to decode UTF-8".to_string())
}

pub fn dpapi_crypt_unprotect_data(mut data_buf: Vec<u8>) -> Result<Vec<u8>, String> {
    println!("{:?}", data_buf.clone());
    println!("{:?}", String::from_utf8_lossy(data_buf.as_slice()).into_owned());
    let buf_ptr = data_buf.as_mut_ptr();
    let buf_len = data_buf.len();
    let mut source = CRYPT_INTEGER_BLOB {
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
            println!("{}", e.clone().to_string());
            e.to_string()
        }).expect("TODO: panic message");

        out_key = Vec::from_raw_parts(
            output.pbData, output.cbData as usize, output.cbData as usize
        );
    }

    /*
    let result = unsafe { result.assume_init() };

    let result_str = String::from_utf8_lossy(unsafe {
            &*slice_from_raw_parts(
                result.pbData,
                result.cbData as usize,
            )
        }
    ).to_string();
     */


    Ok(out_key)
}

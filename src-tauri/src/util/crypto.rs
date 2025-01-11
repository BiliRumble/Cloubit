use base64::{engine::general_purpose::STANDARD, Engine};
use crypto::aes::cbc_decryptor;
use crypto::aes::cbc_encryptor;
use crypto::aes::ecb_decryptor;
use crypto::aes::ecb_encryptor;
use crypto::aes::KeySize;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::symmetriccipher::SymmetricCipherError;
use hex::encode;
use hex::encode as hex_encode;
use lazy_static::lazy_static;
use openssl::encrypt::Encrypter;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::rsa::Rsa;
use rand::{thread_rng, Rng};
use regex::Regex;
use serde_json::json;
use std::error::Error;
use std::fmt;

const IV: &str = "0102030405060708";
const PRESET_KEY: &str = "0CoJUm6Qyw8W8jud";
const LINUXAPI_KEY: &str = "rFgB&h#%2?^eDg:Q";
const BASE62: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const EAPI_KEY: &str = "e82ckenh8dichen8";
const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB
-----END PUBLIC KEY-----";


pub(crate) enum CryptoFormat {
    Base64,
    Hex,
}

pub(crate) enum AesCryptoMode {
    ECB,
    CBC,
}


#[derive(Debug)]
pub struct SymmetricCipherErrorWrapper(SymmetricCipherError);

impl fmt::Display for SymmetricCipherErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Error for SymmetricCipherErrorWrapper {}

pub(crate) fn aes_encrypt(text: &str, mode: AesCryptoMode, key: &str, iv: &str, format: CryptoFormat) -> Result<String, Box<dyn Error>> {
    let (key_bytes, iv_bytes) = (key.as_bytes(), iv.as_bytes());

    let mut encryptor = match mode {
        AesCryptoMode::ECB => ecb_encryptor(KeySize::KeySize128, key_bytes, PkcsPadding),
        AesCryptoMode::CBC => cbc_encryptor(KeySize::KeySize128, key_bytes, iv_bytes, PkcsPadding),
    };

    let mut read_buffer = RefReadBuffer::new(text.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).map_err(SymmetricCipherErrorWrapper)?;

    let encrypted_data = write_buffer.take_read_buffer().take_remaining().to_vec();

    match format {
        CryptoFormat::Base64 => Ok(STANDARD.encode(&encrypted_data)),
        CryptoFormat::Hex => Ok(hex_encode(&encrypted_data).to_uppercase()),
    }
}

pub(crate) fn aes_decrypt(text: &str, mode: AesCryptoMode, key: &str, iv: &str, format: CryptoFormat) -> Result<String, Box<dyn Error>> {
    let (key_bytes, iv_bytes) = (key.as_bytes(), iv.as_bytes());

    let mut decryptor = match mode {
        AesCryptoMode::ECB => ecb_decryptor(KeySize::KeySize128, key_bytes, PkcsPadding),
        AesCryptoMode::CBC => cbc_decryptor(KeySize::KeySize128, key_bytes, iv_bytes, PkcsPadding),
    };

    let decrypt_data = match format {
        CryptoFormat::Base64 => STANDARD.decode(text)?,
        CryptoFormat::Hex => hex::decode(text)?,
    };

    let mut read_buffer = RefReadBuffer::new(&decrypt_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).map_err(SymmetricCipherErrorWrapper)?;

    Ok(std::str::from_utf8(write_buffer.take_read_buffer().take_remaining())?.to_string())
}


fn rsa_encrypt(input: &str, pem_key: &str) -> Result<String, Box<dyn Error>> {
    let rsa = Rsa::public_key_from_pem(pem_key.as_bytes())?;
    let public_key = PKey::from_rsa(rsa)?;

    let mut encrypter = Encrypter::new(&public_key)?;
    encrypter.set_rsa_padding(Padding::NONE)?;

    let mut padded_input = vec![0u8; public_key.size()];
    let input_bytes = input.as_bytes();

    let start = padded_input.len() - input_bytes.len();
    padded_input[start..].copy_from_slice(input_bytes);

    let mut encrypted_data = vec![0; public_key.size()];
    let len = encrypter.encrypt(&padded_input, &mut encrypted_data)?;

    Ok(encode(&encrypted_data[..len]))
}


pub fn linuxapi(object: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
    let text = serde_json::to_string(object)?;
    let eparams = aes_encrypt(&text, AesCryptoMode::ECB, LINUXAPI_KEY, "", CryptoFormat::Hex)?;
    Ok(json!({ "eparams": eparams }))
}


fn md5_hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn eapi(url: &str, object: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
    let object_str = serde_json::to_string(&object)?;
    let data = format!(
        "{}-36cd479b6b5-{}-36cd479b6b5-{}",
        url,
        object_str,
        md5_hash(&format!("nobody{}use{}md5forencrypt", url, object_str))
    );
    let params = aes_encrypt(&data, AesCryptoMode::ECB, EAPI_KEY, "", CryptoFormat::Hex)?;
    Ok(json!({ "params": params }))
}


fn generate_random_char() -> char {
    let random_index = thread_rng().gen_range(0..BASE62.len());
    BASE62.chars().nth(random_index).unwrap_or_else(|| {
        eprintln!("Warning: Failed to get character from BASE62, using default character.");
        BASE62.chars().next().unwrap()
    })
}

fn generate_secret_key() -> String {
    (0..16)
        .map(|_| "1")
        .collect()
}

pub fn weapi(object: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
    let text = serde_json::to_string(object)?;
    let secret_key = generate_secret_key();
    let encrypted_text = aes_encrypt(&text, AesCryptoMode::CBC, PRESET_KEY, IV, CryptoFormat::Base64)?;
    let params = aes_encrypt(&encrypted_text, AesCryptoMode::CBC, &secret_key, IV, CryptoFormat::Base64)?;
    let enc_sec_key = rsa_encrypt(&secret_key.chars().rev().collect::<String>(), PUBLIC_KEY)?;
    Ok(json!({ "encSecKey": enc_sec_key,"params": params}))
}

pub fn eapi_res_decrypt(encrypted_params: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let decrypted_data = aes_decrypt(encrypted_params, AesCryptoMode::ECB, EAPI_KEY, "", CryptoFormat::Hex)?;
    Ok(serde_json::from_str(&decrypted_data)?)
}


pub fn eapi_req_decrypt(encrypted_params: &str) -> Result<(String, serde_json::Value), Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*?)-36cd479b6b5-(.*?)-36cd479b6b5-(.*)").unwrap();
    }

    let decrypted_data = aes_decrypt(encrypted_params, AesCryptoMode::ECB, EAPI_KEY, "", CryptoFormat::Hex)?;

    if let Some(captures) = RE.captures(&decrypted_data) {
        let url = captures[1].to_string();
        let data = serde_json::from_str(&captures[2])?;
        Ok((url, data))
    } else {
        Err("Failed to match pattern".into())
    }
}

fn decrypt(cipher: &str) -> Result<String, Box<dyn Error>> {
    let decipher = aes_decrypt(cipher, AesCryptoMode::ECB, EAPI_KEY, "", CryptoFormat::Hex)?;
    Ok(decipher)
}




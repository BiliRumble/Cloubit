use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit, KeyIvInit};
use aes::Aes128;
use base64::{engine::general_purpose::STANDARD, Engine};
use hex::encode;
use hex::encode as hex_encode;
use openssl::encrypt::Encrypter;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::rsa::Rsa;
use serde_json::json;
use std::error::Error;
use std::fmt;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128EcbEnc = ecb::Encryptor<Aes128>;

const IV: &str = "0102030405060708";
const PRESET_KEY: &str = "0CoJUm6Qyw8W8jud";
const EAPI_KEY: &str = "e82ckenh8dichen8";
const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB
-----END PUBLIC KEY-----";

pub(crate) enum CryptoFormat {
    Base64,
    Hex,
}

pub(crate) enum AesCryptoMode {
    Ecb,
    Cbc,
}

#[derive(Debug)]
pub struct AesEncryptionError(String);

impl fmt::Display for AesEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AES encryption error: {}", self.0)
    }
}

impl Error for AesEncryptionError {}

pub(crate) fn aes_encrypt(
    text: &str,
    mode: AesCryptoMode,
    key: &str,
    iv: &str,
    format: CryptoFormat,
) -> Result<String, Box<dyn Error>> {
    let key_bytes = if key.len() < 16 {
        let mut padded_key = vec![0u8; 16];
        padded_key[..key.len()].copy_from_slice(key.as_bytes());
        padded_key
    } else {
        key.as_bytes()[..16].to_vec()
    };

    let iv_bytes = if iv.len() < 16 {
        let mut padded_iv = vec![0u8; 16];
        padded_iv[..iv.len()].copy_from_slice(iv.as_bytes());
        padded_iv
    } else {
        iv.as_bytes()[..16].to_vec()
    };

    let encrypted_data = match mode {
        AesCryptoMode::Ecb => {
            let cipher = Aes128EcbEnc::new_from_slice(&key_bytes)
                .map_err(|e| AesEncryptionError(e.to_string()))?;
            cipher.encrypt_padded_vec_mut::<Pkcs7>(text.as_bytes())
        }
        AesCryptoMode::Cbc => {
            let cipher = Aes128CbcEnc::new_from_slices(&key_bytes, &iv_bytes)
                .map_err(|e| AesEncryptionError(e.to_string()))?;
            cipher.encrypt_padded_vec_mut::<Pkcs7>(text.as_bytes())
        }
    };

    match format {
        CryptoFormat::Base64 => Ok(STANDARD.encode(&encrypted_data)),
        CryptoFormat::Hex => Ok(hex_encode(&encrypted_data).to_uppercase()),
    }
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

fn md5_hash(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

pub fn eapi(url: &str, object: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
    let object_str = serde_json::to_string(&object)?;
    let data = format!(
        "{}-36cd479b6b5-{}-36cd479b6b5-{}",
        url,
        object_str,
        md5_hash(&format!("nobody{}use{}md5forencrypt", url, object_str))
    );
    let params = aes_encrypt(&data, AesCryptoMode::Ecb, EAPI_KEY, "", CryptoFormat::Hex)?;
    Ok(json!({ "params": params }))
}

fn generate_secret_key() -> String {
    "1".repeat(16)
}

pub fn weapi(object: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
    let text = serde_json::to_string(object)?;
    let secret_key = generate_secret_key();
    let encrypted_text = aes_encrypt(
        &text,
        AesCryptoMode::Cbc,
        PRESET_KEY,
        IV,
        CryptoFormat::Base64,
    )?;
    let params = aes_encrypt(
        &encrypted_text,
        AesCryptoMode::Cbc,
        &secret_key,
        IV,
        CryptoFormat::Base64,
    )?;
    let enc_sec_key = rsa_encrypt(&secret_key.chars().rev().collect::<String>(), PUBLIC_KEY)?;
    Ok(json!({ "encSecKey": enc_sec_key,"params": params}))
}

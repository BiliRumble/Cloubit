use base64::{engine::general_purpose::STANDARD, Engine};
use crypto::aes::cbc_encryptor;
use crypto::aes::ecb_encryptor;
use crypto::aes::KeySize;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::symmetriccipher::SymmetricCipherError;
use hex::encode;
use hex::encode as hex_encode;
use openssl::encrypt::Encrypter;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::rsa::Rsa;
use serde_json::json;
use std::error::Error;
use std::fmt;

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

pub(crate) fn aes_encrypt(
    text: &str,
    mode: AesCryptoMode,
    key: &str,
    iv: &str,
    format: CryptoFormat,
) -> Result<String, Box<dyn Error>> {
    let (key_bytes, iv_bytes) = (key.as_bytes(), iv.as_bytes());

    let mut encryptor = match mode {
        AesCryptoMode::ECB => ecb_encryptor(KeySize::KeySize128, key_bytes, PkcsPadding),
        AesCryptoMode::CBC => cbc_encryptor(KeySize::KeySize128, key_bytes, iv_bytes, PkcsPadding),
    };

    let mut read_buffer = RefReadBuffer::new(text.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    encryptor
        .encrypt(&mut read_buffer, &mut write_buffer, true)
        .map_err(SymmetricCipherErrorWrapper)?;

    let encrypted_data = write_buffer.take_read_buffer().take_remaining().to_vec();

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

fn generate_secret_key() -> String {
    "1".repeat(16)
}

pub fn weapi(object: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
    let text = serde_json::to_string(object)?;
    let secret_key = generate_secret_key();
    let encrypted_text = aes_encrypt(
        &text,
        AesCryptoMode::CBC,
        PRESET_KEY,
        IV,
        CryptoFormat::Base64,
    )?;
    let params = aes_encrypt(
        &encrypted_text,
        AesCryptoMode::CBC,
        &secret_key,
        IV,
        CryptoFormat::Base64,
    )?;
    let enc_sec_key = rsa_encrypt(&secret_key.chars().rev().collect::<String>(), PUBLIC_KEY)?;
    Ok(json!({ "encSecKey": enc_sec_key,"params": params}))
}

use rand::rngs::OsRng;
use rand::{rng, Rng, TryRngCore};
use std::sync::LazyLock;

static DEVICE_ID: LazyLock<String> = LazyLock::new(generate_device_id);

const SDEVICEID_MD5: &[&str] = &[
    "F8EF0C9966B28293CC8D6415CCD93549",
    "B74BAAFF42C04B74CE59EDA3A7B31C8E",
    "279C344F8D33AB4D6DD29C3461348A1F",
    "3D9383C7C9865E15E56511F30B20ED79",
    "DDB7505B124874F7B7A7E5650170ACCA",
    "F8936CF42612B2A72B8041F5F6149031",
    "2A3C522A2FD4787870970C6220B9CD61",
    "901790707FF094D6572F73FE15F449C7",
];

fn generate_device_id() -> String {
    let mut random_bytes = [0u8; 20];
    match OsRng.try_fill_bytes(&mut random_bytes) {
        Ok(_) => {
            let suffix = SDEVICEID_MD5[rng().random_range(0..SDEVICEID_MD5.len())];
            format!("{}{}", hex::encode(random_bytes).to_uppercase(), suffix)
        }
        Err(_) => {
            "4UCKU2DEV1CE1DS1I45I7D7841F18CF238EC76AB6C92AFCBEE0E".to_string()
        }
    }
}

pub fn get_device_id() -> &'static str {
    &DEVICE_ID
}

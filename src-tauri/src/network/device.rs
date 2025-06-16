use rand::rngs::OsRng;
use rand::TryRngCore;
use std::sync::OnceLock;

static DEVICE_ID: OnceLock<String> = OnceLock::new();

const DEVICE_ID_SUFFIXES: &[&str] = &[
    "F8EF0C9966B28293CC8D6415CCD93549",
    "B74BAAFF42C04B74CE59EDA3A7B31C8E",
    "279C344F8D33AB4D6DD29C3461348A1F",
    "3D9383C7C9865E15E56511F30B20ED79",
    "DDB7505B124874F7B7A7E5650170ACCA",
    "F8936CF42612B2A72B8041F5F6149031",
    "2A3C522A2FD4787870970C6220B9CD61",
    "901790707FF094D6572F73FE15F449C7",
    "7322A82E19B0EF72C5D6F2FAD0FD843D",
];

pub fn get_device_id() -> &'static String {
    DEVICE_ID.get_or_init(|| {
        let mut random_bytes = [0u8; 20];
        match OsRng.try_fill_bytes(&mut random_bytes) {
            Ok(_) => {
                let mut index_bytes = [0u8; 1];
                let _ = OsRng.try_fill_bytes(&mut index_bytes);
                let index = (index_bytes[0] as usize) % DEVICE_ID_SUFFIXES.len();
                let suffix = DEVICE_ID_SUFFIXES[index];
                format!("{}{}", "967EC0141EDBE6529FF7", suffix)
            }
            Err(_) => {
                // 应该永远都不会用上
                format!(
                    "{}7D7841F18CF238EC76AB6C92AFCBEE0E",
                    hex::encode(random_bytes).to_uppercase()
                )
            }
        }
    })
}

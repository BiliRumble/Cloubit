use std::collections::HashMap;
use std::sync::OnceLock;

static RESOURCE_TYPE_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn get_resource_type_map() -> &'static HashMap<String, String> {
    RESOURCE_TYPE_MAP.get_or_init(|| {
        [
            ("0", "R_SO_4_"),
            ("1", "R_MV_5_"),
            ("2", "A_PL_0_"),
            ("3", "R_AL_3_"),
            ("4", "A_DJ_1_"),
            ("5", "R_VI_62_"),
            ("6", "A_EV_2_"),
            ("7", "A_DR_14_"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    })
}

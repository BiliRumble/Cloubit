pub struct Os {
    pub os: String,
    pub appver: String,
    pub osver: String,
    pub channel: String,
}

pub fn choose_os(os_name: &str) -> Os {
    match os_name {
        "android" => Os {
            os: "android".to_string(),
            appver: "8.20.20.231215173437".to_string(),
            osver: "14".to_string(),
            channel: "xiaomi".to_string(),
        },
        "iphone" => Os {
            os: "iPhone OS".to_string(),
            appver: "9.0.90".to_string(),
            osver: "16.2".to_string(),
            channel: "distribution".to_string(),
        },
        "pc" => Os {
            os: "pc".to_string(),
            appver: "3.0.18.203152".to_string(),
            osver: "Microsoft-Windows-10-Professional-build-24631-64bit".to_string(),
            channel: "netease".to_string(),
        },
        "linux" => Os {
            os: "linux".to_string(),
            appver: "1.2.1.0428".to_string(),
            osver: "Deepin 20.9".to_string(),
            channel: "netease".to_string(),
        },
        _ => Os {
            os: "iPhone OS".to_string(),
            appver: "9.0.90".to_string(),
            osver: "16.2".to_string(),
            channel: "distribution".to_string(),
        },
    }
}

use log::info;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigYml {
    pub provider: String,
    pub server: ConfigServer,
    pub device: ConfigDevice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigServer {
    pub host: String,
    pub tls: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDevice {
    pub productKey: String,
    pub deviceName: String,
    pub deviceSecret: String,
}

pub fn load_config() -> ConfigYml {
    let filePath = dirs::home_dir()
        .unwrap()
        .join(".iot-echo")
        .join("config.yaml")
        .display()
        .to_string();
    let ctn = fs::read_to_string(filePath).unwrap();
    let config: ConfigYml = serde_yaml::from_str(&ctn).expect("error yaml");

    info!("config.yaml = {:#?}", config);
    config
}

mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        println!("{:?}", load_config());
    }
}

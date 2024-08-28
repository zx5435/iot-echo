use crate::model::ConfigYml;
use log::info;
use std::fs;

pub fn loadConfig() -> ConfigYml {
    let filePath = dirs::home_dir().unwrap().join(".iot-echo").join("config.yaml").display().to_string();
    let ctn = fs::read_to_string(filePath).unwrap();
    let config: ConfigYml = serde_yaml::from_str(&ctn).expect("error yaml");

    info!("config = {:#?}", config);
    config
}

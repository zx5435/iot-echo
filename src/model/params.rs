use log::info;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsYml {
    pub channels: Vec<Channel>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub network: String,
    pub endpoint: String,
    pub protocol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub namespace: Option<String>,
    pub name: String,
    pub value: Option<String>,
    pub script: Option<String>,
    pub channelRefName: Option<String>,
    pub slaveId: Option<String>,
    pub address: Option<String>,
    pub datatype: Option<String>,
}

pub fn load_params() -> ParamsYml {
    let filePath = dirs::home_dir()
        .unwrap()
        .join(".iot-echo")
        .join("params.yaml")
        .display()
        .to_string();
    let ctn = fs::read_to_string(filePath).unwrap();
    let config: ParamsYml = serde_yaml::from_str(&ctn).expect("error yaml");

    info!("params.yaml = {:#?}", config);
    config
}

mod tests {
    use super::*;

    #[test]
    fn test_load_params() {
        println!("{:#?}", load_params());
    }
}

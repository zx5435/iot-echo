use log::{error, info};
use paho_mqtt::{Client, Message};
use std::process;

pub fn rpcHandle(message: Message, rpcPrefix2: String, insMqtt: &Client) {
    let topic = message.topic();

    let uuid = topic[topic.rfind('/').unwrap() + 1..].to_string();
    let topicRet = format!("{}{}", rpcPrefix2, uuid);
    let payload = std::str::from_utf8(message.payload()).unwrap_or("err");
    info!("uuid = {} payload = {}", uuid, payload);

    let body = match payload {
        "LoadConfigInputs" => "any config",
        "ip addr" => {
            "$ ipconfig.exe
Windows IP Configuration
Unknown adapter Clash:
   Connection-specific DNS Suffix  . :
"
        }
        _ => r#"{"message":"not match"}"#,
    };

    let msg = Message::new(topicRet.clone(), body, 0);
    if let Err(e) = insMqtt.publish(msg) {
        error!("Failed to send topic: {:?}", e);
        process::exit(1);
    }
}

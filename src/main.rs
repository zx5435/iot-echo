#![allow(non_snake_case)]

mod model;
mod utils;

extern crate paho_mqtt as mqtt;

use chrono::Local;
use dirs;
use env_logger::Builder;
use log::{info, LevelFilter};
use model::vo::ConfigYml;
use std::io::Write;
use std::{fs, process, thread, time::Duration};

// Subscribe to a single topic.
fn subscribe_topic(cli: &mqtt::Client, topic: &str) {
    if let Err(e) = cli.subscribe(topic, 0) {
        info!("Failed to subscribes topic: {:?}", e);
        process::exit(1);
    }
}

// Reconnect
fn try_to_reconnect(cli: &mqtt::Client) -> bool {
    info!("Disconnected. Waiting to retry connection");
    let cnt = 1;
    while cnt != 4 {
        thread::sleep(Duration::from_millis(cnt * 5000));
        if cli.reconnect().is_ok() {
            info!("Reconnected");
            return true;
        }
    }
    info!("Failed to reconnected.");
    false
}

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} - [{:>20}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S%Z"),
                record.level(),
                record.module_path().unwrap(),
                // record.file().unwrap(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let configFile = dirs::home_dir().unwrap().join(".iot-echo").join("config.yaml").display().to_string();
    let f = fs::read_to_string(configFile).unwrap();
    let config: ConfigYml = serde_yaml::from_str(&f).expect("error yaml");
    info!("config = {:#?}", config);
    let (connOpt1, connOpt2) = utils::getConnOpt(&config);

    // Create a mqtt client.
    let insMqtt = mqtt::Client::new(connOpt1).unwrap_or_else(|err| {
        info!("Failed to create mqtt client: {:?}", err);
        process::exit(1);
    });

    // Define consumer
    let rx = insMqtt.start_consuming();

    // Connect and wait for results.
    if let Err(e) = insMqtt.connect(connOpt2) {
        info!("Failed to connect: {:?}", e);
        process::exit(1);
    }

    // Subscribe to topic "/${productKey}/${deviceName}/user/get"
    let sub_topic = format!("/{}/{}/user/get", config.device.productKey, config.device.deviceName);
    subscribe_topic(&insMqtt, &sub_topic);
    info!("sub topic {}", sub_topic);

    // Publish to topic "/${productKey}/${deviceName}/user/get"
    let pub_topic = format!("/{}/{}/user/update", config.device.productKey, config.device.deviceName);
    let payload = "{\"LightSwitch\":1}".to_string();
    let msg = mqtt::Message::new(pub_topic.clone(), payload.clone(), 0);
    if let Err(e) = insMqtt.publish(msg) {
        info!("Failed to subscribes topic: {:?}", e);
        process::exit(1);
    }
    info!("pub topic {}", pub_topic.clone());
    info!("start receiving...");

    for message in rx.iter() {
        if let Some(message) = message {
            info!("{}", message);
        }

        if !insMqtt.is_connected() {
            if try_to_reconnect(&insMqtt) {
                //
            } else {
                info!("failed to reconnect...");
                break;
            }
        }
    }

    // Disconnect and exit now.
    if insMqtt.is_connected() {
        info!("Disconnecting");
        insMqtt.disconnect(None).unwrap();
    }
    info!("exit");
}

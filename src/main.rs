#![allow(non_snake_case)]

mod model;
mod utils;

extern crate paho_mqtt as mqtt;

use chrono::Local;
use ctrlc;
use dirs;
use env_logger::Builder;
use log::{error, info, LevelFilter};
use model::vo::ConfigYml;
use mqtt::{Client, Message};
use std::io::Write;
use std::sync::mpsc::channel;
use std::{fs, process, thread, time::Duration};
use std::sync::Arc;

// Subscribe to a single topic.
fn subscribe_topic(cli: &Client, topic: &str) {
    if let Err(e) = cli.subscribe(topic, 0) {
        info!("Failed to subscribes topic: {:?}", e);
        process::exit(1);
    }
}

// Reconnect
fn _try_to_reconnect(cli: &Client) -> bool {
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
                record.file().unwrap(),
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
    let insMqtt = Client::new(connOpt1).unwrap_or_else(|err| {
        info!("Failed to create mqtt client: {:?}", err);
        process::exit(1);
    });

    // Define consumer
    let rxConsumer = insMqtt.start_consuming();

    // Connect and wait for results.
    if let Err(e) = insMqtt.connect(connOpt2) {
        info!("Failed to connect: {:?}", e);
        process::exit(1);
    }

    // Subscribe to topic "/${productKey}/${deviceName}/user/get"
    let pk = config.device.productKey;
    let dn = config.device.deviceName;
    let sub_topic = format!("/{}/{}/user/get", pk, dn);
    subscribe_topic(&insMqtt, &sub_topic);
    info!("sub topic {}", sub_topic);

    // Publish to topic "/${productKey}/${deviceName}/user/get"
    let topic_update = format!("/{}/{}/user/update", pk, dn);
    let payload = "{\"cpu\":23}".to_string();
    let msg = Message::new(topic_update.clone(), payload.clone(), 0);
    if let Err(e) = insMqtt.publish(msg) {
        info!("Failed to subscribes topic: {:?}", e);
        process::exit(1);
    }
    info!("pub topic {}", topic_update.clone());
    info!("start receiving...");

    let (txExit, rxExit) = channel();
    ctrlc::set_handler(move || {
        txExit.send(()).expect("Could not send signal on channel.");
    })
    .expect("Error setting Ctrl-C handler");

    // let t0 = Arc::new(&insMqtt);
    // let t1 = Arc::clone(&t0);
    // let t2 = Arc::clone(&t0);

    thread::spawn(move || {
        for message in rxConsumer.iter() {
            if let Some(message) = message {
                info!("{}", message);

                let rpcPrefix1 = format!("/sys/{}/{}/rrpc/request/", pk, dn);
                let rpcPrefix2 = format!("/sys/{}/{}/rrpc/response/", pk, dn);
                if message.topic().contains(&rpcPrefix1) {
                    rpcHandle(message, rpcPrefix2, &insMqtt);
                }
            }
        }
    });
    thread::spawn(move || {
        rxExit.recv().expect("Could not receive from channel.");
        // if insMqtt.is_connected() {
        //     info!("Disconnecting");
        //     insMqtt.disconnect(None).unwrap();
        // }
        info!("sig exit");
        thread::sleep(Duration::from_secs(3));
        process::exit(1);
    });

    loop {
        // if !insMqtt.is_connected() {
        //     if try_to_reconnect(&insMqtt) {
        //         //
        //     } else {
        //         info!("failed to reconnect...");
        //         break;
        //     }
        // }
        let msg = Message::new(topic_update.clone(), r#"{"cpu": 9}"#, 0);
        // if let Err(e) = insMqtt.publish(msg) {
        //     error!("Failed to send topic: {:?}", e);
        //     process::exit(1);
        // }

        info!("3s to send");
        thread::sleep(Duration::from_secs(3));
    }
}

fn rpcHandle(message: Message, rpcPrefix2: String, insMqtt: &Client) {
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

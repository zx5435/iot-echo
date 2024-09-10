#![allow(non_snake_case)]

// extern crate paho_mqtt as mqtt;

mod handle;
mod model;
mod utils;

use crate::handle::rpcHandle;
use crate::utils::{getConn, get_cpu_mem, loadConfig};
use chrono::Local;
use ctrlc;
use env_logger::Builder;
use log::{info, LevelFilter};
use paho_mqtt::{Client, Message};
use std::collections::HashMap;
use std::io::Write;
use std::sync::mpsc::channel;
use std::{process, thread, time::Duration};

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

    let config = loadConfig();
    let (connCfg, connAuth) = getConn(&config);

    // Create a mqtt client.
    let mqIns = Client::new(connCfg).unwrap_or_else(|err| {
        info!("Failed to create mqtt client: {:?}", err);
        process::exit(1);
    });

    // Define consumer
    let mqReceiver = mqIns.start_consuming();

    // Connect and wait for results.
    if let Err(e) = mqIns.connect(connAuth) {
        info!("Failed to connect: {:?}", e);
        process::exit(1);
    }

    // Subscribe to topic "/${productKey}/${deviceName}/user/get"
    let pk = config.device.productKey;
    let dn = config.device.deviceName;
    let sub_topic = format!("/{}/{}/user/get", pk, dn);
    subscribe_topic(&mqIns, &sub_topic);
    info!("sub topic {}", sub_topic);

    // Publish to topic "/${productKey}/${deviceName}/user/get"
    let topic_update = format!("/{}/{}/user/update", pk, dn);
    let payload = "{\"cpu\":23}".to_string();
    let msg = Message::new(topic_update.clone(), payload.clone(), 0);
    if let Err(e) = mqIns.publish(msg) {
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
        for message in mqReceiver.iter() {
            if let Some(message) = message {
                info!("{}", message);

                let rpcPrefix1 = format!("/sys/{}/{}/rrpc/request/", pk, dn);
                let rpcPrefix2 = format!("/sys/{}/{}/rrpc/response/", pk, dn);
                if message.topic().contains(&rpcPrefix1) {
                    rpcHandle(message, rpcPrefix2, &mqIns);
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
        let (cpu, mem) = get_cpu_mem();
        let mut map = HashMap::new();
        map.insert("cpu", cpu);
        map.insert("mem", mem);
        let json_str = serde_json::to_string(&map).unwrap();
        println!("{}", json_str);

        let msg = Message::new(topic_update.clone(), json_str, 0);
        // if let Err(e) = insMqtt.publish(msg) {
        //     error!("Failed to send topic: {:?}", e);
        //     process::exit(1);
        // }

        info!("msg {}", msg);
        thread::sleep(Duration::from_secs(5));
    }
}

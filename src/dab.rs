pub mod app_telemetry;
pub mod applications;
pub mod device;
pub mod device_telemetry;
pub mod health_check;
pub mod input;
pub mod operations;
pub mod output;
pub mod system;
pub mod version;
pub mod voice;
use crate::device::rdk as hw_specific;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    process, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use paho_mqtt::{
    message::Message, message::MessageBuilder, properties::Properties, properties::PropertyCode,
    Client, ConnectOptionsBuilder, CreateOptionsBuilder,
};

fn subscribe(cli: &Client, device_id: &str) -> bool {
    let topic = "dab/".to_owned() + device_id + "/#";
    if let Err(e) = cli.subscribe(&topic, 0) {
        println!("Error subscribing topic: {:?}", e);
        return false;
    }
    return true;
}

fn connect(cli: &Client) -> bool {
    // Connect and wait for it to complete or fail.

    let conn_opts = ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        return false;
    }
    return true;
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    appId: Option<String>,
    force: Option<bool>,
    keyCode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SimpleResponse {
    pub status: u16,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Default, Serialize, Deserialize)]
pub enum NotificationLevel {
    #[default]
    info,
    warn,
    debug,
    trace,
    error,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Messages {
    pub timestamp: u64,
    level: NotificationLevel,
    ip: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
}

fn decode_request(packet: Message) -> String {
    String::from_utf8(packet.payload().to_vec()).unwrap()
}

pub fn run(
    mqtt_host: String,
    mqtt_port: u16,
    mut handlers: HashMap<String, Box<dyn FnMut(String) -> Result<String, String>>>,
) {
    // Get the device ID
    let device_id = hw_specific::interface::get_device_id();

    // Connect to the MQTT broker
    let create_opts = CreateOptionsBuilder::new()
        .server_uri(mqtt_host + ":" + &mqtt_port.to_string())
        .mqtt_version(5)
        .finalize();

    let cli = Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let rx = cli.start_consuming();

    if connect(&cli) == false {
        process::exit(1);
    }

    // Broadcast a message to dab/<device-id>/messages topic:

    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let ip_address = hw_specific::interface::get_ip_address();

    let msg = serde_json::to_string(&Messages {
        timestamp: unix_time,
        level: NotificationLevel::info,
        ip: ip_address,
        message: "DAB started successfully".to_string(),
    })
    .unwrap();

    let msg_tx = MessageBuilder::new()
        .topic("dab/".to_string() + &device_id + "/messages")
        .payload(msg)
        .qos(0)
        .finalize();

    if let Err(e) = cli.publish(msg_tx) {
        println!("Error sending message: {:?}", e);
    }

    // subscribe to all topics starting with `dab/<device-id>/`
    if subscribe(&cli, &device_id) == false {
        process::exit(1);
    }
    // Process incoming messages
    println!("Processing requests...");
    for msg_rx in rx.iter() {
        if let Some(packet) = msg_rx {
            let result: String;
            let function_topic = std::string::String::from(packet.topic());
            let substring = "dab/".to_owned() + &device_id + "/";
            let operation = function_topic.replace(&substring, "");

            let rx_properties = packet.properties().clone();
            let msg = decode_request(packet);

            match handlers.get_mut(&operation) {
                Some(callback) => {
                    println!("OK: {}", operation);
                    // println!("MSG: {}",msg);
                    result = match callback(msg) {
                        Ok(r) => r,
                        Err(e) => serde_json::to_string(&ErrorResponse {
                            status: 500,
                            error: e,
                        })
                        .unwrap(),
                    }
                }
                // If we can't get the proper handler, then this function is not implemented (yet)
                _ => {
                    println!("ERROR: {}", operation);
                    result = serde_json::to_string(&SimpleResponse { status: 501 }).unwrap();
                }
            }

            let response_topic = rx_properties.get_string(PropertyCode::ResponseTopic);
            let correlation_data = rx_properties.get_string(PropertyCode::CorrelationData);
            if let Some(r) = response_topic {
                let mut msg_prop = Properties::new();
                if let Some(c) = correlation_data {
                    // Set topic properties
                    if let Err(e) = msg_prop.push_val(PropertyCode::CorrelationData, c) {
                        println!("Error setting Msg Properties: {:?}", e);
                        process::exit(1);
                    }
                }
                // Publish to a topic
                let msg_tx = MessageBuilder::new()
                    .topic(r)
                    .payload(result)
                    .qos(0)
                    .properties(msg_prop)
                    .finalize();
                let tok = cli.publish(msg_tx);
                if let Err(e) = tok {
                    println!("Error sending message: {:?}", e);
                }
            }
        } else if !cli.is_connected() {
            println!("Connection lost. Waiting to retry connection");
            loop {
                thread::sleep(Duration::from_millis(5000));
                if connect(&cli) == false {
                    process::exit(1);
                } else {
                    println!("Successfully reconnected");
                    if subscribe(&cli, &device_id) == false {
                        process::exit(1);
                    }
                    break;
                }
            }
        }
    }
}

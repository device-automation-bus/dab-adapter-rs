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
use std::sync::{Arc, Mutex, RwLock};
use std::{
    collections::HashMap,
    process, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use paho_mqtt::{
    message::Message, message::MessageBuilder, properties::Properties, properties::PropertyCode,
    Client, ConnectOptionsBuilder, CreateOptionsBuilder,
};

use device_telemetry::DeviceTelemetry;
// use device_telemetry::start::device_telemetry_start_process;
// use device_telemetry::stop::device_telemetry_stop_process;

fn subscribe(cli: &Client, device_id: &str) -> bool {
    let topics = vec![
        "dab/".to_owned() + device_id + "/#",
        "dab/discovery".to_owned(),
    ];
    if let Err(e) = cli.subscribe_many(&topics, &[0, 0]) {
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

fn mqtt_publish(shared_cli: &Arc<RwLock<Client>>, topic: String, correlation_data: String, message: String)
{
    let mut msg_prop = Properties::new();
    // Set topic properties
    if let Err(e) = msg_prop.push_val(PropertyCode::CorrelationData, correlation_data) {
        println!("Error setting Msg Properties: {:?}", e);
        process::exit(1);
    }
    let msg_tx = MessageBuilder::new()
        .topic(topic)
        .payload(message)
        .qos(0)
        .properties(msg_prop)
        .finalize();

    let cli = shared_cli.read().unwrap();
    let tok = cli.publish(msg_tx);
    if let Err(e) = tok {
        println!("Error sending message: {:?}", e);
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    appId: Option<String>,
    force: Option<bool>,
    keyCode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NotImplemented {
    pub status: u16,
    pub error: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DabResponse {
    pub status: u16,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiscoveryResponse {
    pub status: u16,
    pub ip: String,
    pub deviceId: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeviceTelemetryStartResponse{
    pub status: u16,
    pub duration: u64,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryMessage{
    pub timestamp: u64,
    pub metric: String,
    pub value: u32,
}


pub type SharedMap =
    HashMap<String, Box<dyn FnMut(String) -> Result<String, String> + Send + Sync>>;

fn process_msg(
    packet: Message,
    shared_cli: Arc<RwLock<Client>>,
    device_id: String,
    ip_address: String,
    shared_map: Arc<RwLock<SharedMap>>,
    dab_mutex: Arc<Mutex<()>>,
    device_telemetry: &mut DeviceTelemetry,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result: String;
    let function_topic = std::string::String::from(packet.topic());
    let substring = "dab/".to_owned() + &device_id + "/";
    let rx_properties = packet.properties().clone();
    let response_topic = rx_properties.get_string(PropertyCode::ResponseTopic);
    let mut correlation_data = "".to_string();

    if let Some(c) = rx_properties.get_string(PropertyCode::CorrelationData) {
        // Set topic properties
        correlation_data = c.clone();
    }

    if function_topic == "dab/discovery" {
        result = serde_json::to_string(&DiscoveryResponse {
            status: 200,
            ip: ip_address.clone(),
            deviceId: device_id.clone(),
        })
        .unwrap();
    } else {
        let operation = function_topic.replace(&substring, "");
        println!("DEBUG: {}", function_topic);
        let msg = String::from_utf8(packet.payload().to_vec()).unwrap();

        let mut write_map = shared_map.write().unwrap();

        match write_map.get_mut(&operation) {
            // If we get the proper handler, then call it
            Some(callback) => {
                println!("OK: {}", operation);

                match dab_mutex.try_lock() {
                    Ok(_guard) => {
                        result = match callback(msg) {
                            Ok(r) => r,
                            Err(e) => serde_json::to_string(&ErrorResponse {
                                status: 500,
                                error: e,
                            })
                            .unwrap(),
                        }
                    }
                    Err(_e) => {
                        println!("Dab busy");
                        result = serde_json::to_string(&NotImplemented {
                            status: 500,
                            error: "Processing previous request".to_string(),
                        })
                        .unwrap();
                    }
                }
            }
            // If we can't get the proper handler, then this is a telemetry operation or is not implemented
            _ => {
                // If the operation is device-telemetry/start, then start the device telemetry thread
                if &operation == "device-telemetry/start" {
                    result = device_telemetry::start::process(msg, device_telemetry).unwrap();
                } else if &operation == "device-telemetry/stop" {
                    result = device_telemetry::stop::process(msg, device_telemetry).unwrap();
                } else {
                    println!("ERROR: {}", operation);
                    result = serde_json::to_string(&NotImplemented {
                        status: 501,
                        error: "Not implemented".to_string(),
                    })
                    .unwrap();
                }
            }
        }
    }

    if let Some(r) = response_topic {
        // Publish to a topic
        mqtt_publish(&shared_cli, r, correlation_data, result);
    }

    Ok(())
}

pub fn run(mqtt_host: String, mqtt_port: u16, shared_map: Arc<RwLock<SharedMap>>) {
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

    // subscribe to all topics starting with `dab/<device-id>/`
    if subscribe(&cli, &device_id) == false {
        process::exit(1);
    }

    let shared_cli = Arc::new(RwLock::new(cli));
    let shared_cli_main = Arc::clone(&shared_cli);
    let cli = shared_cli_main.read().unwrap();
    let dab_mutex = Arc::new(Mutex::new(()));

    // Broadcast a message to dab/<device-id>/messages topic:

    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let ip_address = hw_specific::interface::get_ip_address();

    let msg = serde_json::to_string(&Messages {
        timestamp: unix_time,
        level: NotificationLevel::info,
        ip: ip_address.clone(),
        message: "DAB started successfully".to_string(),
    })
    .unwrap();

    let shared_cli = Arc::clone(&shared_cli);
    mqtt_publish(&shared_cli, "dab/".to_string() + &device_id.clone() + "/messages", "".to_string(), msg);

    // Start the device telemetry thread
    let shared_cli = Arc::clone(&shared_cli);
    let mut device_telemetry = DeviceTelemetry::new(device_id.clone(), Arc::clone(&shared_cli));

    // Process incoming messages
    println!("Ready to process DAB requests");
    for msg_rx in rx.iter() {
        if let Some(packet) = msg_rx {
            // Spawn a new task to process the received message
            let shared_map = Arc::clone(&shared_map);
            let shared_cli = Arc::clone(&shared_cli);
            let dab_mutex = Arc::clone(&dab_mutex);
            process_msg(
                packet.clone(),
                shared_cli,
                device_id.clone(),
                ip_address.clone(),
                shared_map,
                dab_mutex,
                &mut device_telemetry,
            ).unwrap();

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
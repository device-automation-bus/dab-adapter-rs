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
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, Ordering}};
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
    let correlation_data = rx_properties.get_string(PropertyCode::CorrelationData);

    if function_topic == "dab/discovery" {
        result = serde_json::to_string(&DiscoveryResponse {
            status: 200,
            ip: ip_address.clone(),
            deviceId: device_id.clone(),
        })
        .unwrap();
    } else {
        let operation = function_topic.replace(&substring, "");
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
                    result = device_telemetry_start_process(msg, device_telemetry).unwrap();
                } else if &operation == "device-telemetry/stop" {
                    result = device_telemetry_stop_process(msg, device_telemetry).unwrap();
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

        let cli = shared_cli.read().unwrap();
        let tok = cli.publish(msg_tx);
        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);
        }
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

    let shared_cli = Arc::new(RwLock::new(cli));
    let shared_cli_main = Arc::clone(&shared_cli);
    let cli = shared_cli_main.read().unwrap();
    let dab_mutex = Arc::new(Mutex::new(()));

    // Start the device telemetry thread
    let mut device_telemetry = DeviceTelemetry::new();

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

// Implement device-telemetry

pub struct DeviceTelemetry {
    enabled: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl DeviceTelemetry {
    pub fn new( ) -> DeviceTelemetry {
        DeviceTelemetry {
            enabled: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    pub fn start(&mut self, period: u64) {
        // If it is already running, stop the instance before creating a new one
        if self.enabled.load(Ordering::Relaxed) {
            self.stop();
        }

        // Start the telemetry thread
        self.enabled.store(true, Ordering::Relaxed);
        let enabled = self.enabled.clone();
        self.handle = Some(thread::spawn(move || {
            while enabled.load(Ordering::Relaxed) {
                println!("Printing from thread every {} ms", period);
                thread::sleep(Duration::from_millis(period));
            }
        }));
    }

    pub fn stop(&mut self) {
        // Stop the telemetry thread
        self.enabled.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}


use serde_json::json;
use crate::dab::device_telemetry::start::StartDeviceTelemetryRequest;
use crate::dab::device_telemetry::start::StartDeviceTelemetryResponse;
#[allow(non_snake_case)]
pub fn device_telemetry_start_process(_packet: String, device_telemetry: &mut DeviceTelemetry) -> Result<String, String> {

    let mut ResponseOperator = StartDeviceTelemetryResponse::default();

    let IncomingMessage = serde_json::from_str(&_packet);

    match IncomingMessage {
        Err(err) => {
            let response = ErrorResponse {
                status: 400,
                error: "Error parsing request: ".to_string() + err.to_string().as_str(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
        _ => (),
    }

    let Dab_Request: StartDeviceTelemetryRequest = IncomingMessage.unwrap();

    device_telemetry.start(Dab_Request.duration);

    ResponseOperator.duration = Dab_Request.duration;

    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

use crate::dab::device_telemetry::stop::StopDeviceTelemetryResponse;
#[allow(non_snake_case)]
pub fn device_telemetry_stop_process(_packet: String, device_telemetry: &mut DeviceTelemetry) -> Result<String, String> {
    let ResponseOperator = StopDeviceTelemetryResponse::default();

    device_telemetry.stop();

    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
pub mod device_telemetry;
pub mod mqtt_client;
pub mod structs;

use mqtt_client::MqttClient;
use mqtt_client::MqttMessage;
use structs::DiscoveryResponse;
use structs::ErrorResponse;
use structs::Messages;
use structs::NotImplemented;
use structs::NotificationLevel;
use structs::TelemetryMessage;

use crate::device::rdk as hw_specific;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub type SharedMap =
    HashMap<String, Box<dyn FnMut(String) -> Result<String, String> + Send + Sync>>;

use device_telemetry::DeviceTelemetry;

pub fn run(mqtt_server: String, mqtt_port: u16, mut function_map: SharedMap) {
    // Get the device ID
    let device_id = match hw_specific::interface::get_device_id(){
        Ok(id) => id,
        Err(e) => {
            println!("RDK: Error getting device ID: {}", e);
            "unknown".to_string()
        }
    };
    println!("DAB Device ID: {}", device_id);

    // Connect to the MQTT broker
    let mut mqtt_client = MqttClient::new(mqtt_server, mqtt_port);
    mqtt_client.start();
    // subscribe to all topics starting with `dab/<device-id>/`
    mqtt_client.subscribe("dab/".to_string() + &device_id + "/#");
    mqtt_client.subscribe("dab/discovery".to_string());

    // Broadcast a message to dab/<device-id>/messages topic:

    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let ip_address = hw_specific::interface::get_ip_address();

    let payload = serde_json::to_string(&Messages {
        timestamp: unix_time,
        level: NotificationLevel::info,
        ip: ip_address.clone(),
        message: "DAB started successfully".to_string(),
    })
    .unwrap();

    let mut zero_vector: Vec<u8> = Vec::new();
    zero_vector.push(0);

    let msg_tx = MqttMessage {
        function_topic: "dab/".to_string() + &device_id.clone() + "/messages",
        response_topic: "".to_string(),
        correlation_data: zero_vector,
        payload: payload.clone(),
    };
    mqtt_client.publish(msg_tx);

    // Start the device telemetry thread
    let mqtt_client_telemetry = mqtt_client.clone();
    let mut device_telemetry = DeviceTelemetry::new(mqtt_client_telemetry, device_id.clone());

    // Infinite loop
    loop {
        // Check for messages
        match mqtt_client.receive() {
            Ok(msg_received) => {
                let function_topic = msg_received.function_topic;
                let response_topic = msg_received.response_topic;
                let correlation_data = msg_received.correlation_data;
                let payload = msg_received.payload;
                // let response: String;

                // Process the message
                let response = if function_topic == "dab/discovery" {
                    println!("OK: {}", function_topic);
                    serde_json::to_string(&DiscoveryResponse {
                        status: 200,
                        ip: ip_address.clone(),
                        deviceId: device_id.clone(),
                    })
                    .unwrap()
                } else {
                    let substring = "dab/".to_owned() + &device_id + "/";
                    let operation = function_topic.replace(&substring, "");

                    if &operation == "messages" {
                        continue;
                    }

                    match function_map.get_mut(&operation) {
                        // If we get the proper handler, then call it
                        Some(callback) => {
                            println!("OK: {}", operation);

                            match callback(payload) {
                                Ok(r) => r,
                                Err(e) => serde_json::to_string(&ErrorResponse {
                                    status: 500,
                                    error: e,
                                })
                                .unwrap(),
                            }
                        }
                        // If we can't get the proper handler, then this is a telemetry operation or is not implemented
                        _ => {
                            // If the operation is device-telemetry/start, then start the device telemetry thread
                            if &operation == "device-telemetry/start" {
                                device_telemetry
                                    .device_telemetry_start_process(payload)
                                    .unwrap()
                            } else if &operation == "device-telemetry/stop" {
                                device_telemetry
                                    .device_telemetry_stop_process(payload)
                                    .unwrap()
                            } else {
                                println!("ERROR: {}", operation);
                                serde_json::to_string(&NotImplemented {
                                    status: 501,
                                    error: operation + " operator not implemented",
                                })
                                .unwrap()
                            }
                        }
                    }
                };

                let msg_tx = MqttMessage {
                    function_topic: response_topic.clone(),
                    response_topic: "".to_string(),
                    correlation_data: correlation_data.clone(),
                    payload: response.clone(),
                };
                // Publish the response
                mqtt_client.publish(msg_tx);
            }
            Err(err) => {
                if let Some(msg) = err {
                    println!("Error: {}", msg);
                }
            }
        }
    }
}

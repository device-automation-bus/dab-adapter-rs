pub mod device_telemetry;
pub mod mqtt_client;
pub mod structs;

use serde_json::json;
use mqtt_client::MqttClient;
use mqtt_client::MqttMessage;
use structs::SharedMap;
use structs::DiscoveryResponse;
use structs::ErrorResponse;
use structs::Messages;
use structs::NotificationLevel;
use structs::RequestTypes;
use structs::TelemetryMessage;

use crate::device::rdk as hw_specific;
use std::time::{SystemTime, UNIX_EPOCH};

use device_telemetry::DeviceTelemetry;

fn call_function(json_str: String,request_type: RequestTypes) -> Result<String,String> {
    match request_type {
        RequestTypes::OperationsListRequest => {
            let dab_request: structs::OperationsListRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::operations::list::process(dab_request)
        },
        RequestTypes::ApplicationListRequest => {
            let dab_request: structs::ApplicationListRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::applications::list::process(dab_request)
        },
        RequestTypes::ApplicationLaunchRequest => {
            let dab_request: structs::LaunchApplicationRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::applications::launch::process(dab_request)
        },
        RequestTypes::ApplicationLaunchWithContentRequest => {
            let dab_request: structs::LaunchApplicationWithContentRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::applications::launch_with_content::process(dab_request)
        },
        RequestTypes::ApplicationGetStateRequest => {
            let dab_request: structs::GetApplicationStateRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::applications::get_state::process(dab_request)
        },
        RequestTypes::ApplicationExitRequest => {
            let dab_request: structs::ExitApplicationRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::applications::exit::process(dab_request)
        },
        RequestTypes::DeviceInfoRequest => {
            let dab_request: structs::DeviceInfoRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::device::info::process(dab_request)
        },
        RequestTypes::SystemRestartRequest => {
            let dab_request: structs::RestartRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::system::restart::process(dab_request)
        },
        RequestTypes::SystemSettingsListRequest => {
            let dab_request: structs::ListSystemSettingsRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::system::settings::list::process(dab_request)
        },
        RequestTypes::SystemSettingsGetRequest => {
            let dab_request: structs::GetSystemSettingsRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::system::settings::get::process(dab_request)
        },
        RequestTypes::SystemSettingsSetRequest => {
            let dab_request: structs::SetSystemSettingsRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::system::settings::set::process(dab_request)
        },
        RequestTypes::InputKeyListRequest => {
            let dab_request: structs::KeyListRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::input::key::list::process(dab_request)
        },
        RequestTypes::InputKeyPressRequest => {
            let dab_request: structs::KeyPressRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::input::key_press::process(dab_request)
        },
        RequestTypes::InputLongKeyPressRequest => {
            let dab_request: structs::LongKeyPressRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::input::long_key_press::process(dab_request)
        },
        RequestTypes::OutputImageRequest => {
            let dab_request: structs::CaptureScreenshotRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::output::image::process(dab_request)
        },
        RequestTypes::HealthCheckGetRequest => {
            let dab_request: structs::HealthCheckRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::health_check::get::process(dab_request)
        },
        RequestTypes::VoiceListRequest => {
            let dab_request: structs::VoiceListRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::voice::list::process(dab_request)
        },
        RequestTypes::VoiceSetRequest => {
            let dab_request: structs::SetVoiceSystemRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::voice::set::process(dab_request)
        },
        RequestTypes::VoiceSendAudioRequest => {
            let dab_request: structs::SendAudioRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::voice::send_audio::process(dab_request)
        },
        RequestTypes::VoiceSendTextRequest => {
            let dab_request: structs::SendTextRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::voice::send_text::process(dab_request)
        },
        RequestTypes::VersionRequest => {
            let dab_request: structs::VersionRequest =
                serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
            hw_specific::version::process(dab_request)
        },
    }
}

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
                        Some(request_type) => {
                            println!("processing: {}", operation);
                            match call_function(payload.clone(), request_type.clone()) {
                                Ok(r) => {
                                    // The request was successful.
                                    let mut response_json = json!(r);
                                    response_json["status"] = json!(200);
                                    response_json.to_string()
                                },
                                Err(e) => serde_json::to_string(&ErrorResponse {
                                    // Internal error. The explanation of the error 
                                    // must be included in the error field of the response.
                                    status: 500,
                                    error: e
                                })
                                .unwrap(),
                            }
                        }
                        // If we can't get the proper handler, then this is a telemetry operation or is not implemented
                        _ => {
                            // If the operation is device-telemetry/start, then start the device telemetry thread
                            let function_response = if &operation == "device-telemetry/start" {
                                let dab_request: Result<structs::StartDeviceTelemetryRequest,String> =
                                    serde_json::from_str(&payload.clone()).map_err(|e| e.to_string());
                                match dab_request {
                                    Ok(r) => {
                                        Ok(device_telemetry
                                            .device_telemetry_start_process(r)
                                            .unwrap())
                                    },
                                    Err(e) => {
                                        println!("ERROR: {}", e);
                                        Err(e)
                                    }
                                }
                            } else if &operation == "device-telemetry/stop" {
                                let dab_request: Result<structs::StopDeviceTelemetryRequest,String> =
                                    serde_json::from_str(&payload.clone()).map_err(|e| e.to_string());
                                match dab_request {
                                    Ok(r) => {
                                        Ok(device_telemetry
                                            .device_telemetry_stop_process(r)
                                            .unwrap())
                                    },
                                    Err(e) => {
                                        println!("ERROR: {}", e);
                                        Err(e)
                                    }
                                }
                            } else {
                                println!("ERROR: {}", operation);
                                Err(operation + " operator not implemented")
                            };

                            match function_response{
                                Ok(r) => {
                                    // The request was successful.
                                    let mut response_json = json!(r);
                                    response_json["status"] = json!(200);
                                    response_json.to_string()
                                },
                                Err(e) => serde_json::to_string(&ErrorResponse {
                                    // Internal error. The explanation of the error 
                                    // must be included in the error field of the response.
                                    status: 500,
                                    error: e
                                })
                                .unwrap(),
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

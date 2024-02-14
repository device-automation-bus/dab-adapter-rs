use serde_json::Value;
pub mod device_telemetry;
pub mod mqtt_client;
pub mod structs;
use crate::device::rdk as hw_specific;
use mqtt_client::{MqttClient, MqttMessage};
use std::time::{SystemTime, UNIX_EPOCH};
use structs::{
    DabError, DabResponse, DiscoveryResponse, ErrorResponse, Messages, NotificationLevel,
    RequestTypes, SharedMap, TelemetryMessage,
};

use device_telemetry::DeviceTelemetry;

fn call_function(json_str: String, request_type: RequestTypes) -> Result<String, DabError> {
    match request_type {
        RequestTypes::OperationsListRequest => {
            let dab_request: structs::OperationsListRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::operations::list::process(dab_request)
        }
        RequestTypes::ApplicationListRequest => {
            let dab_request: structs::ApplicationListRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::applications::list::process(dab_request)
        }
        RequestTypes::ApplicationLaunchRequest => {
            let dab_request: structs::LaunchApplicationRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::applications::launch::process(dab_request)
        }
        RequestTypes::ApplicationLaunchWithContentRequest => {
            let dab_request: structs::LaunchApplicationWithContentRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::applications::launch_with_content::process(dab_request)
        }
        RequestTypes::ApplicationGetStateRequest => {
            let dab_request: structs::GetApplicationStateRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::applications::get_state::process(dab_request)
        }
        RequestTypes::ApplicationExitRequest => {
            let dab_request: structs::ExitApplicationRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::applications::exit::process(dab_request)
        }
        RequestTypes::DeviceInfoRequest => {
            let dab_request: structs::DeviceInfoRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::device::info::process(dab_request)
        }
        RequestTypes::SystemRestartRequest => {
            let dab_request: structs::RestartRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::system::restart::process(dab_request)
        }
        RequestTypes::SystemSettingsListRequest => {
            let dab_request: structs::ListSystemSettingsRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::system::settings::list::process(dab_request)
        }
        RequestTypes::SystemSettingsGetRequest => {
            let dab_request: structs::GetSystemSettingsRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::system::settings::get::process(dab_request)
        }
        RequestTypes::SystemSettingsSetRequest => {
            let dab_request: structs::SetSystemSettingsRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::system::settings::set::process(dab_request)
        }
        RequestTypes::InputKeyListRequest => {
            let dab_request: structs::KeyListRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::input::key::list::process(dab_request)
        }
        RequestTypes::InputKeyPressRequest => {
            let dab_request: structs::KeyPressRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::input::key_press::process(dab_request)
        }
        RequestTypes::InputLongKeyPressRequest => {
            let dab_request: structs::LongKeyPressRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::input::long_key_press::process(dab_request)
        }
        RequestTypes::OutputImageRequest => {
            let dab_request: structs::CaptureScreenshotRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::output::image::process(dab_request)
        }
        RequestTypes::HealthCheckGetRequest => {
            let dab_request: structs::HealthCheckRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::health_check::get::process(dab_request)
        }
        RequestTypes::VoiceListRequest => {
            let dab_request: structs::VoiceListRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::voice::list::process(dab_request)
        }
        RequestTypes::VoiceSetRequest => {
            let dab_request: structs::SetVoiceSystemRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::voice::set::process(dab_request)
        }
        RequestTypes::VoiceSendAudioRequest => {
            let dab_request: structs::SendAudioRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::voice::send_audio::process(dab_request)
        }
        RequestTypes::VoiceSendTextRequest => {
            let dab_request: structs::SendTextRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::voice::send_text::process(dab_request)
        }
        RequestTypes::VersionRequest => {
            let dab_request: structs::VersionRequest =
                serde_json::from_str(&json_str).map_err(|e| DabError::Err400(e.to_string()))?;
            hw_specific::version::process(dab_request)
        }
    }
}

pub fn run(mqtt_server: String, mqtt_port: u16, mut function_map: SharedMap) {
    // Get the device ID
    let device_id = match hw_specific::interface::get_device_id() {
        Ok(id) => id,
        Err(_) => {
            println!("RDK: Error getting device ID");
            // Without a valid Device ID; DAB functionality cannot progress.
            // Exit now so that systemd can restart it again.
            std::process::exit(0x00);
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

                let substring = "dab/".to_owned() + &device_id + "/";

                // Process the message
                let response = if payload.trim().is_empty() {
                    Err(DabError::Err400(
                        "No payload. Dab Request needs to be at least empty {}".to_string(),
                    ))
                } else if function_topic == "dab/discovery" {
                    println!("OK: {}", function_topic);
                    Ok(serde_json::to_string(&DiscoveryResponse {
                        ip: ip_address.clone(),
                        deviceId: device_id.clone(),
                    })
                    .unwrap())
                } else {
                    let operation = function_topic.replace(&substring, "");

                    if &operation == "messages" {
                        continue;
                    }

                    match function_map.get_mut(&operation) {
                        // If we get the proper handler, then call it
                        Some(request_type) => {
                            println!("processing: {}", operation);
                            call_function(payload.clone(), request_type.clone())
                        }
                        // If we can't get the proper handler, then this is a telemetry operation or is not implemented
                        _ => {
                            // If the operation is device-telemetry/start, then start the device telemetry thread
                            if &operation == "device-telemetry/start" {
                                let dab_request: Result<
                                    structs::StartDeviceTelemetryRequest,
                                    DabError,
                                > = serde_json::from_str(&payload.clone())
                                    .map_err(|e| DabError::Err400(e.to_string()));
                                match dab_request {
                                    Ok(r) => device_telemetry.device_telemetry_start_process(r),
                                    Err(e) => Err(e),
                                }
                            } else if &operation == "device-telemetry/stop" {
                                let dab_request: Result<
                                    structs::StopDeviceTelemetryRequest,
                                    DabError,
                                > = serde_json::from_str(&payload.clone())
                                    .map_err(|e| DabError::Err400(e.to_string()));
                                match dab_request {
                                    Ok(r) => device_telemetry.device_telemetry_stop_process(r),
                                    Err(e) => Err(e),
                                }
                            } else {
                                println!("ERROR: {}", operation);
                                Err(DabError::Err501(operation + " operator not implemented"))
                            }
                        }
                    }
                };

                let payload = match response {
                    Ok(r) => {
                        // The request was successful.
                        let template = DabResponse { status: 200 };
                        let dab_json =
                            serde_json::to_value(template).expect("Error serializing DabResponse");
                        // Parse the JSON string
                        let mut dab_response: Value =
                            serde_json::from_str(&r).expect("Error parsing JSON string");
                        if dab_response.is_object() {
                            for (key, value) in dab_json.as_object().unwrap() {
                                dab_response[key] = value.clone();
                            }
                        }
                        dab_response.to_string()
                    }
                    Err(e) => match e {
                        DabError::Err400(msg) => {
                            // The request was not successful.
                            serde_json::to_string(&ErrorResponse {
                                // Bad request. The explanation of the error
                                // must be included in the error field of the response.
                                status: 400,
                                error: msg,
                            })
                            .unwrap()
                        }
                        DabError::Err500(msg) => {
                            // The request was not successful.
                            serde_json::to_string(&ErrorResponse {
                                // Internal error. The explanation of the error
                                // must be included in the error field of the response.
                                status: 500,
                                error: msg,
                            })
                            .unwrap()
                        }
                        DabError::Err501(msg) => {
                            // The request was not successful.
                            serde_json::to_string(&ErrorResponse {
                                // Internal error. The explanation of the error
                                // must be included in the error field of the response.
                                status: 501,
                                error: msg,
                            })
                            .unwrap()
                        }
                    },
                };

                let msg_tx = MqttMessage {
                    function_topic: response_topic.clone(),
                    response_topic: "".to_string(),
                    correlation_data: correlation_data.clone(),
                    payload: payload.clone(),
                };
                // Publish the response
                mqtt_client.publish(msg_tx);
                println!("Publishing response: {} {:?}", response_topic.clone().replace(&substring, ""), payload.as_str());
            }
            Err(err) => {
                if let Some(msg) = err {
                    println!("Error: {}", msg);
                }
            }
        }
    }
}

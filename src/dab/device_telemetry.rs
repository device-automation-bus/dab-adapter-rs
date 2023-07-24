use crate::dab::{mqtt_client::MqttMessage, ErrorResponse, MqttClient, TelemetryMessage};
use crate::hw_specific::interface::get_device_memory;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Implement device-telemetry
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopDeviceTelemetryRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopDeviceTelemetryResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartDeviceTelemetryRequest {
    pub duration: u64,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartDeviceTelemetryResponse {
    pub duration: u64,
}

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct DeviceTelemetry {
    enabled: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
    mqtt_client: MqttClient,
    device_id: String,
}

impl DeviceTelemetry {
    pub fn new(mqtt_client: MqttClient, device_id: String) -> DeviceTelemetry {
        DeviceTelemetry {
            enabled: Arc::new(AtomicBool::new(false)),
            handle: None,
            mqtt_client: mqtt_client,
            device_id: device_id,
        }
    }

    pub fn start<'a>(&'a mut self, period: u64) {
        // If it is already running, stop the instance before creating a new one
        let enabled = self.enabled.clone();
        if enabled.load(Ordering::Relaxed) {
            self.stop();
        }

        // Start the telemetry thread
        self.enabled.store(true, Ordering::Relaxed);
        let enabled = self.enabled.clone();
        let device_id = self.device_id.clone();
        let mqtt_client = self.mqtt_client.clone();

        self.handle = Some(thread::spawn(move || {
            while enabled.load(Ordering::Relaxed) {
                let memory = match get_device_memory() {
                    Ok(mem) => mem,
                    _ => 0,
                };

                let payload = serde_json::to_string(&TelemetryMessage {
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    metric: "memory".to_string(),
                    value: memory,
                })
                .unwrap();

                let msg_tx = MqttMessage {
                    function_topic: "dab/".to_string() + &device_id + "/telemetry",
                    response_topic: "".to_string(),
                    correlation_data: "".to_string(),
                    payload: payload.clone(),
                };

                mqtt_client.publish(msg_tx);
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

    #[allow(non_snake_case)]
    pub fn device_telemetry_start_process(&mut self, packet: String) -> Result<String, String> {
        let mut ResponseOperator = StartDeviceTelemetryResponse::default();

        let IncomingMessage = serde_json::from_str(&packet);

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

        self.start(Dab_Request.duration);

        ResponseOperator.duration = Dab_Request.duration;

        let mut ResponseOperator_json = json!(ResponseOperator);
        ResponseOperator_json["status"] = json!(200);
        Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
    }

    #[allow(non_snake_case)]
    pub fn device_telemetry_stop_process(&mut self, _packet: String) -> Result<String, String> {
        let ResponseOperator = StopDeviceTelemetryResponse::default();

        self.stop();

        let mut ResponseOperator_json = json!(ResponseOperator);
        ResponseOperator_json["status"] = json!(200);
        Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
    }
}

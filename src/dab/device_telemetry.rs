use crate::dab::structs::DabError;
use crate::dab::structs::StartDeviceTelemetryRequest;
use crate::dab::structs::StartDeviceTelemetryResponse;
use crate::dab::structs::StopDeviceTelemetryRequest;
use crate::dab::structs::StopDeviceTelemetryResponse;
use crate::dab::{mqtt_client::MqttMessage, MqttClient, TelemetryMessage};
use crate::hw_specific::connectivity::rdk::get_device_memory;
use crate::hw_specific::connectivity::rdk::get_device_cpu;

use std::{
    sync::{Arc, Mutex, Condvar},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[allow(dead_code)]
struct ThreadContext {
    mutex: Mutex<bool>,
    cond: Condvar,
}

#[allow(dead_code)]
pub struct DeviceTelemetry {
    handle: Option<thread::JoinHandle<()>>,
    context: Arc<ThreadContext>,
    mqtt_client: MqttClient,
    device_id: String,
}

#[allow(dead_code)]
impl DeviceTelemetry {
    pub fn new(mqtt_client: MqttClient, device_id: String) -> DeviceTelemetry {
        DeviceTelemetry {
            handle: None,
            context: Arc::new(ThreadContext {
                mutex: Mutex::new(false),
                cond: Condvar::new()
            }),
            mqtt_client: mqtt_client,
            device_id: device_id,
        }
    }

    pub fn start<'a>(&'a mut self, period: u64) {
        // If it is already running, stop the instance before creating a new one
        self.stop();

        // Start the telemetry thread
        let mut enabled = self.context.mutex.lock().unwrap();
        assert!(*enabled == false);
        *enabled = true;
        let context = self.context.clone();
        let device_id = self.device_id.clone();
        let mqtt_client = self.mqtt_client.clone();

        self.handle = Some(thread::spawn(move || {
            let ThreadContext {mutex, cond} = &*context;

            loop {
                let metrics = [("memory", get_device_memory()), ("cpu", get_device_cpu())];

                let zero_vector = vec![0];

                for (metric_name, metric_value) in &metrics {
                    let value = match metric_value {
                        Ok(val) => *val,
                        _ => 0,
                    };

                    let payload = Self::get_telemetry_payload(metric_name, value).unwrap();

                    let msg_tx = MqttMessage {
                        function_topic: format!("dab/{}/device-telemetry/metrics", device_id),
                        response_topic: "".to_string(),
                        correlation_data: zero_vector.clone(),
                        payload,
                    };

                    mqtt_client.publish(msg_tx);
                }

                if cond.wait_timeout_while(
                    mutex.lock().unwrap(),
                    Duration::from_millis(period),
                    |&mut enabled| enabled
                ).unwrap().1.timed_out() == false {
                    break;
                }
            }
        }));
    }

    pub fn stop(&mut self) {
        // Stop the telemetry thread
        let mut enabled = self.context.mutex.lock().unwrap();
        *enabled = false;
        if let Some(handle) = self.handle.take() {
            self.context.cond.notify_one();
            drop(enabled);
            handle.join().unwrap();
        }
    }

    #[allow(non_snake_case)]
    pub fn device_telemetry_start_process(
        &mut self,
        _dab_request: StartDeviceTelemetryRequest,
    ) -> Result<String, DabError> {
        let mut ResponseOperator = StartDeviceTelemetryResponse::default();

        self.start(_dab_request.duration);

        ResponseOperator.duration = _dab_request.duration;

        Ok(serde_json::to_string(&ResponseOperator).unwrap())
    }

    #[allow(non_snake_case)]
    pub fn device_telemetry_stop_process(
        &mut self,
        _dab_request: StopDeviceTelemetryRequest,
    ) -> Result<String, DabError> {
        let ResponseOperator = StopDeviceTelemetryResponse::default();

        self.stop();

        Ok(serde_json::to_string(&ResponseOperator).unwrap())
    }

    fn get_telemetry_payload(metric: &str, value: u32) -> Result<String, serde_json::Error> {
        let message = TelemetryMessage {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metric: metric.to_string(),
            value,
        };
        serde_json::to_string(&message)
    }
}

pub mod start;
pub mod stop;
use crate::hw_specific::interface::get_device_memory;
use crate::dab::{TelemetryMessage, mqtt_publish, RwLock, Client};

// Implement device-telemetry

use std::{
	sync::{Arc, atomic::{AtomicBool, Ordering}},
   thread,
   time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct DeviceTelemetry {
	device_id: String,
	shared_cli: Arc<RwLock<Client>>,
   enabled: Arc<AtomicBool>,
   handle: Option<thread::JoinHandle<()>>,
}

impl DeviceTelemetry {
    pub fn new(device_id: String, shared_cli: Arc<RwLock<Client>> ) -> DeviceTelemetry {
        DeviceTelemetry {
				device_id: device_id,
				shared_cli:	shared_cli,
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
		  let cli = self.shared_cli.clone();
		  let device_id = self.device_id.clone();
        self.handle = Some(thread::spawn(move || {
            while enabled.load(Ordering::Relaxed) {
					
					let memory = match get_device_memory() {
						Ok(mem) => mem,
						_ => 0,
					};

					let msg = serde_json::to_string(&TelemetryMessage {
						timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
						metric: "memory".to_string(),
						value: memory,
					}).unwrap();

					mqtt_publish(&cli, "dab/".to_string() + &device_id + "/device-telemetry/metrics", "".to_string(), msg);
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


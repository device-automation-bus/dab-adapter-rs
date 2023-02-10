use clap::Parser;
mod device;
use device::rdk as hw_specific;
mod dab;
use std::collections::HashMap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opt {
    /// The MQTT broker host name or IP (default: localhost)
    #[clap(short, long, value_parser, value_name = "MQTT_HOST")]
    broker: Option<String>,
    /// The MQTT broker port (default: 1883)
    #[clap(short, long, value_parser, value_name = "MQTT_PORT")]
    port: Option<u16>,
    /// The device host name or IP (default: localhost)
    #[clap(short, long, value_parser, value_name = "DEVICE")]
    device: Option<String>,
}

pub fn main() {
    let opt = Opt::parse();
    let mqtt_host = opt.broker.unwrap_or(String::from("localhost"));
    let mqtt_port = opt.port.unwrap_or(1883);
    let device = opt
        .device
        .unwrap_or(String::from("http://localhost:9998/jsonrpc"));

    hw_specific::interface::init(device);


    let mut handlers: HashMap<String, Box<dyn FnMut(String) -> Result<String, String>>> =
        HashMap::new();
    handlers.insert(
        "operations/list".to_string(),
        Box::new(hw_specific::operations::list::process),
    );
    handlers.insert(
        "applications/list".to_string(),
        Box::new(hw_specific::applications::list::process),
    );
    handlers.insert(
        "applications/launch".to_string(),
        Box::new(hw_specific::applications::launch::process),
    );
    // handlers.insert(
    //     "applications/launch-with-content".to_string(),
    //     Box::new(hw_specific::applications::launch_with_content::process),
    // );
    handlers.insert(
        "applications/get-state".to_string(),
        Box::new(hw_specific::applications::get_state::process),
    );
    handlers.insert(
        "applications/exit".to_string(),
        Box::new(hw_specific::applications::exit::process),
    );
    handlers.insert(
        "device/info".to_string(),
        Box::new(hw_specific::device::info::process),
    );
    handlers.insert(
        "system/restart".to_string(),
        Box::new(hw_specific::system::restart::process),
    );
    // handlers.insert(
    //     "system/settings/list".to_string(),
    //     Box::new(hw_specific::system::settings::list::process),
    // );
    // handlers.insert(
    //     "system/settings/get".to_string(),
    //     Box::new(hw_specific::system::settings::get::process),
    // );
    // handlers.insert(
    //     "system/settings/set".to_string(),
    //     Box::new(hw_specific::system::settings::set::process),
    // );
    handlers.insert(
        "input/key/list".to_string(),
        Box::new(hw_specific::input::key::list::process),
    );
    handlers.insert(
        "input/key-press".to_string(),
        Box::new(hw_specific::input::key_press::process),
    );
    // handlers.insert(
    //     "input/long-key-press".to_string(),
    //     Box::new(hw_specific::input::long_key_press::process),
    // );
    handlers.insert(
        "output/image".to_string(),
        Box::new(hw_specific::output::image::process),
    );
    // handlers.insert(
    //     "device-telemetry/start".to_string(),
    //     Box::new(hw_specific::device_telemetry::start::process),
    // );
    // handlers.insert(
    //     "device-telemetry/stop".to_string(),
    //     Box::new(hw_specific::device_telemetry::stop::process),
    // );
    // handlers.insert(
    //     "app-telemetry/start".to_string(),
    //     Box::new(hw_specific::app_telemetry::start::process),
    // );
    // handlers.insert(
    //     "app-telemetry/stop".to_string(),
    //     Box::new(hw_specific::app_telemetry::stop::process),
    // );
    // handlers.insert(
    //     "health-check/get".to_string(),
    //     Box::new(hw_specific::health_check::get::process),
    // );
    // handlers.insert(
    //     "voice/list".to_string(),
    //     Box::new(hw_specific::voice::list::process),
    // );
    // handlers.insert(
    //     "voice/set".to_string(),
    //     Box::new(hw_specific::voice::set::process),
    // );
    // handlers.insert(
    //     "voice/send-audio".to_string(),
    //     Box::new(hw_specific::voice::send_audio::process),
    // );
    // handlers.insert(
    //     "voice/send-text".to_string(),
    //     Box::new(hw_specific::voice::send_text::process),
    // );
    handlers.insert(
        "version".to_string(),
        Box::new(hw_specific::version::process),
    );

    dab::run(mqtt_host, mqtt_port, handlers)
}

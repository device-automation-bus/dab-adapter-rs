use clap::Parser;
mod device;
pub use device::rdk as hw_specific;
mod dab;
use dab::structs::RequestTypes;
use dab::structs::SharedMap;
use std::collections::HashMap;
use std::thread;

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
    /// Print the version information
    #[clap(short, long, value_parser, value_name = "VERSION")]
    version: bool,
    /// To exit based on path file (/opt/dab-enable) status.
    #[clap(short, long, value_parser, value_name = "RETIRE")]
    retire: Option<bool>,
    /// Print RDK messages to stdout
    #[clap(long, value_parser, value_name = "DEBUG")]
    debug: Option<bool>,
}

fn fd_monitor_thread() {
    use notify::{event, Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
    use std::path::Path;
    use std::process;
    use std::time::Duration;

    static MONITORPATH: &str = "/opt";
    println!("Monitoring changes of {}/dab-enable", MONITORPATH);
    let monitor_path = Path::new(MONITORPATH);

    let (tx, rx) = std::sync::mpsc::channel();
    let config = Config::default().with_poll_interval(Duration::from_secs(5));
    let mut watcher: Box<dyn Watcher> = Box::new(RecommendedWatcher::new(tx, config).unwrap());

    watcher
        .watch(&monitor_path, RecursiveMode::Recursive)
        .unwrap();

    'fd_wait_loop: for data in rx {
        match data {
            Ok(event) => {
                if event.kind == EventKind::Remove(event::RemoveKind::File)
                    && (event.paths.len() > 0)
                {
                    for i in &event.paths {
                        let rm_file = i.as_path().display().to_string();
                        let monitor_file = monitor_path.display().to_string() + "/dab-enable";
                        if monitor_file.eq(&rm_file) {
                            println!("MATCH: {:?} {:?}", rm_file, monitor_file);
                            break 'fd_wait_loop;
                        }
                    }
                }
            }
            Err(error) => println!("DATA_ER error: {:?}", error),
        }
    }
    println!("Clean-Up triggered.");
    let _ = watcher.unwatch(&monitor_path);
    process::exit(0x00);
}

pub fn main() {
    let opt = Opt::parse();
    let mqtt_host = opt.broker.unwrap_or(String::from("localhost"));
    let mqtt_port = opt.port.unwrap_or(1883);
    let device_ip = opt.device.unwrap_or(String::from("localhost"));
    let create_retire_thread = opt.retire.unwrap_or(false);
    let debug = opt.debug.unwrap_or(false);

    println!("DAB<->RDK Adapter ({:?} - {:?})", env!("VERGEN_BUILD_SEMVER"), env!("VERGEN_GIT_SHA_SHORT"));
    
    // Initialize the device
    hw_specific::interface::init(&device_ip, debug);

    // Register the handlers
    let mut handlers: SharedMap = HashMap::new();

    handlers.insert(
        "operations/list".to_string(),
        RequestTypes::OperationsListRequest,
    );
    handlers.insert(
        "applications/list".to_string(),
        RequestTypes::ApplicationListRequest,
    );
    handlers.insert(
        "applications/launch".to_string(),
        RequestTypes::ApplicationLaunchRequest,
    );
    handlers.insert(
        "applications/launch-with-content".to_string(),
        RequestTypes::ApplicationLaunchWithContentRequest,
    );
    handlers.insert(
        "applications/get-state".to_string(),
        RequestTypes::ApplicationGetStateRequest,
    );
    handlers.insert(
        "applications/exit".to_string(),
        RequestTypes::ApplicationExitRequest,
    );
    handlers.insert("device/info".to_string(), RequestTypes::DeviceInfoRequest);
    handlers.insert(
        "system/restart".to_string(),
        RequestTypes::SystemRestartRequest,
    );
    handlers.insert(
        "system/settings/list".to_string(),
        RequestTypes::SystemSettingsListRequest,
    );
    handlers.insert(
        "system/settings/get".to_string(),
        RequestTypes::SystemSettingsGetRequest,
    );
    handlers.insert(
        "system/settings/set".to_string(),
        RequestTypes::SystemSettingsSetRequest,
    );
    handlers.insert(
        "input/key/list".to_string(),
        RequestTypes::InputKeyListRequest,
    );
    handlers.insert(
        "input/key-press".to_string(),
        RequestTypes::InputKeyPressRequest,
    );
    handlers.insert(
        "input/long-key-press".to_string(),
        RequestTypes::InputLongKeyPressRequest,
    );
    handlers.insert("output/image".to_string(), RequestTypes::OutputImageRequest);
    handlers.insert(
        "health-check/get".to_string(),
        RequestTypes::HealthCheckGetRequest,
    );
    handlers.insert("voice/list".to_string(), RequestTypes::VoiceListRequest);
    handlers.insert("voice/set".to_string(), RequestTypes::VoiceSetRequest);
    handlers.insert(
        "voice/send-audio".to_string(),
        RequestTypes::VoiceSendAudioRequest,
    );
    handlers.insert(
        "voice/send-text".to_string(),
        RequestTypes::VoiceSendTextRequest,
    );
    handlers.insert("version".to_string(), RequestTypes::VersionRequest);

    if create_retire_thread {
        let _handle = thread::Builder::new()
            .name("ExitPathMonitor".to_string())
            .spawn(move || {
                fd_monitor_thread();
            });
    }
    dab::run(mqtt_host, mqtt_port, handlers);
}

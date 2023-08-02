use clap::Parser;
mod device;
pub use device::rdk as hw_specific;
mod dab;
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
    /// To exit based on path file (/run/dab-enable) status.
    #[clap(short, long, value_parser, value_name = "RETIRE")]
    retire: Option<bool>,
}

// The file `built.rs` was placed there by cargo and `build.rs`
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn fd_monitor_thread() {
    use std::process;
    use std::path::Path;
    use notify::{ Config, event, EventKind, RecommendedWatcher, RecursiveMode, Watcher };
    use std::time::Duration;

    static MONITORPATH: &str = "/run";
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
            Ok (event) => {
                if event.kind == EventKind::Remove(event::RemoveKind::File) && (event.paths.len() > 0) {
                    for i in &event.paths {
                        let rm_file = i.as_path().display().to_string();
                        let monitor_file = monitor_path.display().to_string()+"/dab-enable";
                        if monitor_file.eq(&rm_file) {
                            println!("MATCH: {:?} {:?}", rm_file, monitor_file);
                            break 'fd_wait_loop;
                        }
                    }
                }
            },
            Err(error) => println!("DATA_ER error: {:?}", error),
        }
    }
    println!("Clean-Up triggered.");
    let _ = watcher.unwatch(&monitor_path);
    process::exit(0x00);
}

fn display_version() {
    // Print various information produced by `built`. See the docs for a full list.

    println!(
        "DAB<->RDK Adapter\nVersion {}, built for {} by {}.",
        built_info::PKG_VERSION,
        built_info::TARGET,
        built_info::RUSTC_VERSION
    );

    if let Some(hash) = built_info::GIT_COMMIT_HASH {
        print!("Git commit: {}", hash);
    }

    match built_info::GIT_HEAD_REF {
        Some(r) => println!(", branch: `{r}`"),
        None => println!(""),
    }

    print!(
        "Built for a {}-CPU, {}-endian architecture. ",
        built_info::CFG_TARGET_ARCH,
        built_info::CFG_ENDIAN
    );

    let built_time = built::util::strptime(built_info::BUILT_TIME_UTC);
    println!(
        "Built on {}",
        built_time.with_timezone(&built::chrono::offset::Local)
    );
}

pub type SharedMap =
    HashMap<String, Box<dyn FnMut(String) -> Result<String, String> + Send + Sync>>;

pub fn main() {
    let opt = Opt::parse();
    let mqtt_host = opt.broker.unwrap_or(String::from("localhost"));
    let mqtt_port = opt.port.unwrap_or(1883);
    let device_ip = opt.device.unwrap_or(String::from("localhost"));
    let create_retire_thread = opt.retire.unwrap_or(false);

    if opt.version {
        display_version();
        return;
    }

    // Initialize the device
    hw_specific::interface::init(&device_ip);

    // Register the handlers
    let mut handlers: SharedMap = HashMap::new();

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
    handlers.insert(
        "applications/launch-with-content".to_string(),
        Box::new(hw_specific::applications::launch_with_content::process),
    );
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
    handlers.insert(
        "system/settings/list".to_string(),
        Box::new(hw_specific::system::settings::list::process),
    );
    handlers.insert(
        "system/settings/get".to_string(),
        Box::new(hw_specific::system::settings::get::process),
    );
    handlers.insert(
        "system/settings/set".to_string(),
        Box::new(hw_specific::system::settings::set::process),
    );
    handlers.insert(
        "input/key/list".to_string(),
        Box::new(hw_specific::input::key::list::process),
    );
    handlers.insert(
        "input/key-press".to_string(),
        Box::new(hw_specific::input::key_press::process),
    );
    handlers.insert(
        "input/long-key-press".to_string(),
        Box::new(hw_specific::input::long_key_press::process),
    );
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
    handlers.insert(
        "voice/send-audio".to_string(),
        Box::new(hw_specific::voice::send_audio::process),
    );
    handlers.insert(
        "voice/send-text".to_string(),
        Box::new(hw_specific::voice::send_text::process),
    );
    handlers.insert(
        "version".to_string(),
        Box::new(hw_specific::version::process),
    );
    if create_retire_thread {
        let _handle = thread::Builder::new().name("ExitPathMonitor".to_string()).spawn(move || { fd_monitor_thread(); });
    }
    dab::run(mqtt_host, mqtt_port, handlers);
}

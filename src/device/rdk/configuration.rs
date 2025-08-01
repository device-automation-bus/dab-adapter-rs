use crate::dab::structs::AudioVolume;
use crate::dab::structs::DabError;
use crate::hw_specific::interface::rdk::{get_device_info, get_rdk_device_id};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::LazyLock;
use std::sync::OnceLock;

#[derive(Debug, Clone, Default, Deserialize)]
struct AppTimeouts {
    cold_launch_timeout_ms: u64,
    resume_launch_timeout_ms: u64,
    exit_to_destroy_timeout_ms: u64,
    exit_to_background_timeout_ms: u64,
}
type AppTimeoutMap = HashMap<String, AppTimeouts>;

#[derive(Deserialize, Debug)]
struct ConfigurationFileSettings {
    supported_languages: Option<Vec<String>>,
    audio_volume_range: Option<AudioVolume>,
}

struct Configuration {
    device_address: String,
    debug: bool,
    rdk_device_id: LazyLock<String>,
    keymap: LazyLock<HashMap<String, u16>>,
    rdk_device_info: LazyLock<HashMap<String, String>>,
    app_lifecycle_timeouts: LazyLock<AppTimeoutMap>,
    configuration_file_settings: LazyLock<ConfigurationFileSettings>,
}
impl Configuration {
    fn new(device_ip: &str, debug: bool) -> Self {
        Configuration {
            device_address: device_ip.to_string(),
            debug: debug,
            rdk_device_id: LazyLock::new(|| get_rdk_device_id().unwrap()),
            keymap: LazyLock::new(|| get_keymap()),
            rdk_device_info: LazyLock::new(|| get_device_info()),
            app_lifecycle_timeouts: LazyLock::new(|| get_app_timeouts()),
            configuration_file_settings: LazyLock::new(|| get_configuration_file_settings()),
        }
    }
}
static DEVICE_SETTINGS: OnceLock<Configuration> = OnceLock::new();

fn get_device_settings() -> &'static Configuration {
    DEVICE_SETTINGS
        .get()
        .expect("Device Settings accessed but not initialized")
}

pub fn get_device_id() -> Result<String, DabError> {
    Ok(get_device_settings().rdk_device_id.clone())
}

pub fn get_ip_address() -> String {
    get_device_settings().device_address.clone()
}

pub fn get_is_debug() -> bool {
    get_device_settings().debug
}

pub fn get_rdk_keys() -> Vec<String> {
    get_device_settings()
        .keymap
        .keys()
        .map(|k| k.to_owned().to_string())
        .collect()
}

pub fn get_keycode(keyname: String) -> Option<&'static u16> {
    get_device_settings().keymap.get(&keyname)
}

// Parameter: propertyname: The property to get the value of.
// Returns the value of the property on success else DabError.
pub fn get_rdk_device_info(propertyname: &str) -> Result<String, DabError> {
    match get_device_settings().rdk_device_info.get(propertyname) {
        Some(val) => Ok(val.clone()),
        None => {
            let error_message = DabError::Err500(format!("No match for property {propertyname}."));
            return Err(error_message);
        }
    }
}

// Function to get lifecycle timeout for an app. After plugin state change how long App implementation/SDK takes to complete the action.
// Parameters: app_name: The app name (lowercase) to get the timeout for, timeout_type: The type of timeout to get.
// Returns the timeout in milliseconds on success else default 2500.
pub fn get_lifecycle_timeout(app_name: &str, timeout_type: &str) -> Option<u64> {
    let timeouts_map = &get_device_settings().app_lifecycle_timeouts;

    timeouts_map
        .get(app_name)
        .and_then(|timeouts| match timeout_type {
            "cold_launch_timeout_ms" => Some(timeouts.cold_launch_timeout_ms),
            "resume_launch_timeout_ms" => Some(timeouts.resume_launch_timeout_ms),
            "exit_to_destroy_timeout_ms" => Some(timeouts.exit_to_destroy_timeout_ms),
            "exit_to_background_timeout_ms" => Some(timeouts.exit_to_background_timeout_ms),
            _ => None,
        })
        .or(Some(2500))
}

pub fn get_supported_languages() -> Vec<String> {
    get_device_settings()
        .configuration_file_settings
        .supported_languages
        .clone()
        .unwrap_or_else(|| vec![String::from("en-US")])
}

pub fn get_audio_volume_range() -> AudioVolume {
    get_device_settings()
        .configuration_file_settings
        .audio_volume_range
        .clone()
        .unwrap_or_else(|| AudioVolume { min: 0, max: 100 })
}

pub fn init(device_ip: &str, debug: bool) {
    let settings = Configuration::new(device_ip, debug);

    if DEVICE_SETTINGS.set(settings).is_err() {
        panic!("Settings already initialized!");
    }

    if debug {
        for (app, timeouts) in get_device_settings().app_lifecycle_timeouts.iter() {
            println!(
                "{:<15} - {:<30} = {:>5}ms.",
                app, "cold_launch_timeout_ms", timeouts.cold_launch_timeout_ms
            );
            println!(
                "{:<15} - {:<30} = {:>5}ms.",
                app, "resume_launch_timeout_ms", timeouts.resume_launch_timeout_ms
            );
            println!(
                "{:<15} - {:<30} = {:>5}ms.",
                app, "exit_to_destroy_timeout_ms", timeouts.exit_to_destroy_timeout_ms
            );
            println!(
                "{:<15} - {:<30} = {:>5}ms.",
                app, "exit_to_background_timeout_ms", timeouts.exit_to_background_timeout_ms
            );
        }
    }
}

fn get_configuration_file_settings() -> ConfigurationFileSettings {
    let config_path = "/etc/dab/settings.json";

    if let Ok(json_file) = read_platform_config_json(config_path) {
        match serde_json::from_str::<ConfigurationFileSettings>(&json_file) {
            Ok(json_object) => {
                println!("Loaded settings: {:?} from: {}", json_object, config_path);
                return json_object;
            }
            Err(error) => {
                eprintln!("Error while parsing {}: {}", config_path, error);
            }
        }
    }

    println!("Using default settings.");
    ConfigurationFileSettings {
        supported_languages: None,
        audio_volume_range: None,
    }
}

fn get_app_timeouts() -> AppTimeoutMap {
    let mut map = AppTimeoutMap::new();

    map.insert(
        "youtube".to_string(),
        AppTimeouts {
            cold_launch_timeout_ms: 6000,
            resume_launch_timeout_ms: 3000,
            exit_to_destroy_timeout_ms: 2500,
            exit_to_background_timeout_ms: 2000,
        },
    );

    match read_platform_config_json("/opt/dab_platform_app_lifecycle.json") {
        Ok(json_file) => match serde_json::from_str::<HashMap<String, AppTimeouts>>(&json_file) {
            Ok(parsed) => {
                for (app, timeouts) in parsed {
                    if matches!(
                        app.as_str(),
                        "youtube" | "netflix" | "primevideo" | "uk.co.bbc.iplayer"
                    ) {
                        map.insert(app, timeouts);
                    }
                }
                println!("Imported platform specified app lifetime configuration file also.");
            }
            Err(e) => {
                println!(
                    "Failed to parse JSON: {} from 'dab_platform_app_lifecycle.json'.",
                    e
                );
            }
        },
        Err(_) => {
            println!("Using default values for app lifecycle timeouts.");
        }
    }
    map
}

// DAB key codes are listed here:
// https://github.com/device-automation-bus/dab-specification-2.0/blob/main/DAB.md#54-input

// Key mapping is referenced from the following source:
// https://github.com/rdkcentral/RDKShell/blob/master/linuxkeys.h

// The keymap that translates DAB key code to RDK Shell key codes may be
// supplied in the /etc/dab/keymap.json file. When this file is not present,
// the default keymap is used. In both cases, the keymap may be updated via
// the /opt/dab_platform_keymap.json file.
//
// The keymap in the files mentioned above must conform to the following format:
/*
    {
        "KEY_EXIT": 27,
        "KEY_STOP": 178,
        "KEY_CHANNEL_UP": 104,
        "KEY_CHANNEL_DOWN": 109,
        "KEY_MENU": 408,
        "KEY_INFO": 0,
        "KEY_GUIDE": 0,
        "KEY_CAPTIONS": 0,
        "KEY_RECORD": 0,
        "KEY_RED": 0,
        "KEY_GREEN": 0,
        "KEY_YELLOW": 0,
        "KEY_BLUE": 0
    }
*/
fn get_keymap() -> HashMap<String, u16> {
    let mut keycode_map = HashMap::new();
    let mut keymap_file_found = false;

    if let Ok(json_file) = read_platform_config_json("/etc/dab/keymap.json") {
        keymap_file_found = true;
        match serde_json::from_str::<HashMap<String, u16>>(&json_file) {
            Ok(new_keymap) => {
                for (key, value) in new_keymap {
                    keycode_map.insert(key, value);
                }
                println!("Loaded keymap from /etc/dab/keymap.json");
            }
            Err(error) => {
                eprintln!("Error while parsing /etc/dab/keymap.json {}", error);
            }
        }
    }

    if keymap_file_found == false {
        keycode_map.insert(String::from("KEY_POWER"), 116);
        keycode_map.insert(String::from("KEY_HOME"), 36);
        keycode_map.insert(String::from("KEY_VOLUME_UP"), 175);
        keycode_map.insert(String::from("KEY_VOLUME_DOWN"), 174);
        keycode_map.insert(String::from("KEY_MUTE"), 173);
        keycode_map.insert(String::from("KEY_UP"), 38);
        keycode_map.insert(String::from("KEY_PAGE_UP"), 33);
        keycode_map.insert(String::from("KEY_PAGE_DOWN"), 34);
        keycode_map.insert(String::from("KEY_RIGHT"), 39);
        keycode_map.insert(String::from("KEY_DOWN"), 40);
        keycode_map.insert(String::from("KEY_LEFT"), 37);
        keycode_map.insert(String::from("KEY_ENTER"), 13);
        keycode_map.insert(String::from("KEY_BACK"), 8);
        keycode_map.insert(String::from("KEY_PLAY"), 13);
        keycode_map.insert(String::from("KEY_PLAY_PAUSE"), 227);
        keycode_map.insert(String::from("KEY_PAUSE"), 19);
        keycode_map.insert(String::from("KEY_REWIND"), 224);
        keycode_map.insert(String::from("KEY_FAST_FORWARD"), 223);
        keycode_map.insert(String::from("KEY_SKIP_REWIND"), 34);
        keycode_map.insert(String::from("KEY_SKIP_FAST_FORWARD"), 33);
        keycode_map.insert(String::from("KEY_0"), 48);
        keycode_map.insert(String::from("KEY_1"), 49);
        keycode_map.insert(String::from("KEY_2"), 50);
        keycode_map.insert(String::from("KEY_3"), 51);
        keycode_map.insert(String::from("KEY_4"), 52);
        keycode_map.insert(String::from("KEY_5"), 53);
        keycode_map.insert(String::from("KEY_6"), 54);
        keycode_map.insert(String::from("KEY_7"), 55);
        keycode_map.insert(String::from("KEY_8"), 56);
        keycode_map.insert(String::from("KEY_9"), 57);

        println!("Default keymap assigned");
    }

    if let Ok(json_file) = read_platform_config_json("/opt/dab_platform_keymap.json") {
        match serde_json::from_str::<HashMap<String, u16>>(&json_file) {
            Ok(new_keymap) => {
                for (key, value) in new_keymap {
                    keycode_map.insert(key, value);
                }
                println!("Added keymap from /opt/dab_platform_keymap.json");
            }
            Err(error) => {
                eprintln!(
                    "Error while parsing /opt/dab_platform_keymap.json {}",
                    error
                );
            }
        }
    }
    keycode_map
}

// Read platform override JSON configs from file
// Optional override configuration; do not panic or break runtime.
pub fn read_platform_config_json(file_path: &str) -> Result<String, DabError> {
    let mut file_content = String::new();
    File::open(file_path)
        .map_err(|e| {
            if e.kind() != std::io::ErrorKind::NotFound {
                println!("Error opening {}: {}", file_path, e);
            }
            DabError::Err500(e.to_string())
        })?
        .read_to_string(&mut file_content)
        .map_err(|e| {
            println!("Error reading {}: {}", file_path, e);
            DabError::Err500(e.to_string())
        })?;
    Ok(file_content)
}

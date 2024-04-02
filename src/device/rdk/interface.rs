use crate::dab::structs::AudioOutputMode;
use crate::dab::structs::DabError;
use futures::executor::block_on;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::{thread, time};
use surf::Client;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use url::Url;

static mut DEVICE_ADDRESS: String = String::new();
static mut DEBUG: bool = false;

pub fn init(device_ip: &str, debug: bool) {
    unsafe {
        DEVICE_ADDRESS.push_str(&device_ip);
        DEBUG = debug;
    }

    if unsafe { DEBUG } {
        for app in APP_LIFECYCLE_TIMEOUTS.keys() {
            for (key, value) in APP_LIFECYCLE_TIMEOUTS.get(app).unwrap() {
                println!("{:<15} - {:<30} = {:>5}ms.", app, key, value);
            }
        }
    }
}

pub fn get_device_id() -> Result<String, DabError> {
    let json_string =
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"org.rdk.System.getDeviceInfo\",\"params\":{\"params\":[\"estb_mac\"]}}".to_string();
    let response = http_post(json_string)?;
    let rdkresponse: serde_json::Value = serde_json::from_str(&response).unwrap();
    let device_id = rdkresponse["result"]["estb_mac"]
        .as_str()
        .ok_or(DabError::Err500(
            "RDK Error: org.rdk.System.getDeviceInfo.result.estb_mac not found".to_string(),
        ))?;
    Ok(device_id.replace(":", "").to_string())
}

pub fn http_download(url: String) -> Result<(), DabError> {
    let client = Client::new();

    let response = block_on(async { client.get(url).await });

    match response {
        Ok(mut r) => {
            let mut file = File::create("/tmp/tts.wav").unwrap();
            let body = block_on(r.body_bytes()).unwrap();
            file.write_all(&body).unwrap();
            return Ok(());
        }
        Err(err) => return Err(DabError::Err500(err.to_string())),
    }
}

pub fn http_post(json_string: String) -> Result<String, DabError> {
    let client = Client::new();
    let rdk_address = format!("http://{}:9998/jsonrpc", unsafe { &DEVICE_ADDRESS });

    unsafe {
        if DEBUG {
            println!("RDK request: {}", json_string);
        }
    }

    let response = block_on(async {
        match client
            .post(rdk_address)
            .body_string(json_string)
            .header("Content-Type", "application/json")
            .await
        {
            Ok(mut response) => {
                match response.body_string().await {
                    Ok(body) => Ok(body),
                    Err(e) => Err(format!("Error while getting the body: {}",e)),
                }
            }
            Err(e) => Err(format!("Error while sending the request: {}",e)),
        }
    });

    match response {
        Ok(r) => {
            let str = r.to_string();

            unsafe {
                if DEBUG {
                    println!("RDK response: {}", str);
                }
            }

            return Ok(str);
        }
        Err(err) => {
            let str = err.to_string();

            unsafe {
                if DEBUG {
                    println!("RDK error: {}", str);
                }
            }

            return Err(DabError::Err500(str));
        }
    }
}

pub async fn ws_open() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, DabError> {
    let rdk_address = format!("ws://{}:9998/jsonrpc", unsafe { &DEVICE_ADDRESS });
    let url = Url::parse(&rdk_address).expect("Invalid WebSocket URL");

    connect_async(url)
        .await
        .map_err(|e| DabError::Err500(format!("Failed to connect: {}", e)))
        .map(|(ws_stream, _)| ws_stream)
}

pub async fn ws_close(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Result<(), DabError> {
    ws_stream
        .close(None)
        .await
        .map_err(|e| DabError::Err500(format!("Failed to close WebSocket: {}", e)))
}

pub async fn ws_send(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    payload: Value,
) -> Result<(), DabError> {
    ws_stream
        .send(Message::Text(payload.to_string()))
        .await
        .map_err(|e| DabError::Err500(format!("Failed to send message: {}", e)))
}

pub async fn ws_receive(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Result<Value, DabError> {
    match timeout(Duration::from_secs(10), ws_stream.next()).await {
        Ok(Some(Ok(message))) => {
            if let Message::Text(text) = message {
                serde_json::from_str(&text)
                    .map_err(|e| DabError::Err500(format!("Invalid JSON: {}", e)))
            } else {
                Err(DabError::Err500("Received a non-text message".to_string()))
            }
        }
        Ok(Some(Err(e))) => Err(DabError::Err500(format!("Error reading message: {:?}", e))),
        Ok(None) => Err(DabError::Err500(
            "The WebSocket stream has been closed by the server".to_string(),
        )),
        Err(_) => Err(DabError::Err500(
            "Timeout occurred while waiting for a response".to_string(),
        )),
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct RdkResponse<T> {
    pub jsonrpc: String,
    pub id: i32,
    pub result: T,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct RdkResult {
    success: bool,
}

pub type RdkResponseSimple = RdkResponse<RdkResult>;

pub fn rdk_request<R: DeserializeOwned>(method: &str) -> Result<R, DabError> {
    #[derive(Serialize)]
    struct RdkNullParams {}

    rdk_request_impl::<RdkNullParams, R>(method, None)
}

pub fn rdk_request_with_params<P: Serialize, R: DeserializeOwned>(
    method: &str,
    params: P,
) -> Result<R, DabError> {
    rdk_request_impl(method, Some(params))
}

fn rdk_request_impl<P: Serialize, R: DeserializeOwned>(
    method: &str,
    params: Option<P>,
) -> Result<R, DabError> {
    #[derive(Serialize)]
    struct RdkRequest<P> {
        jsonrpc: String,
        id: i32,
        method: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        params: Option<P>,
    }

    static mut JSONRPC_ID: i32 = 1;

    let id;

    unsafe {
        id = JSONRPC_ID;
        JSONRPC_ID += 1;
    }

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id,
        method: method.into(),
        params,
    };
    let json_string = serde_json::to_string(&request).unwrap();
    let response = http_post(json_string)?;

    let val: serde_json::Value = match serde_json::from_str(&response) {
        Ok(val) => val,
        Err(e) => return Err(DabError::Err500(e.to_string())),
    };

    if val["error"] != serde_json::Value::Null {
        return Err(DabError::Err500(
            val["error"]["message"].as_str().unwrap().into(),
        ));
    } else if !val["result"].is_null() && val["result"]["success"].is_boolean() {
        if !val["result"]["success"].as_bool().unwrap() {
            return Err(DabError::Err500(format!("{} failed", method)));
        }
    }

    let res: R = match serde_json::from_value(val) {
        Ok(res) => res,
        Err(e) => return Err(DabError::Err500(e.to_string())),
    };

    Ok(res)
}

// Function to activate a service.
// Parameters: service: The service to activate.
// Returns Ok on success else DabError.
pub fn service_activate(service: String) -> Result<(), DabError> {
    //#########Controller.1.activate#########
    let activate_payload = json!({
        "jsonrpc":"2.0",
        "id":1,
        "method":"Controller.1.activate",
        "params":{
            "callsign":service.clone()
        }
    }).to_string();
    let response = http_post(activate_payload)?;
    let response_value: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| DabError::Err500(format!("Failed to parse response: {}", e)))?;
    if response_value.get("result").is_none() {
        return Err(DabError::Err500(format!("Key 'result' not found in response for method 'Controller.1.activate'.")));
    }
    thread::sleep(time::Duration::from_millis(200));
    if get_service_state(service.as_str())?.to_lowercase() != "activated" {
        return Err(DabError::Err500(format!("Failed to activate service '{}' after 200ms.", service)));
    }
    Ok(())
}

// Function to deactivate a service.
// Parameters: service: The service to deactivate.
// Returns Ok on success else DabError.
pub fn service_deactivate(service: String) -> Result<(), DabError> {
    //#########Controller.1.deactivate#########
    let activate_payload = json!({
        "jsonrpc":"2.0",
        "id":1,
        "method":"Controller.1.deactivate",
        "params":{
            "callsign":service.clone()
        }
    }).to_string();
    let response = http_post(activate_payload)?;
    let response_value: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| DabError::Err500(format!("Failed to parse response: {}", e)))?;
    if response_value.get("result").is_none() {
        return Err(DabError::Err500(format!("Key 'result' not found in response for method 'Controller.1.activate'.")));
    }
    thread::sleep(time::Duration::from_millis(200));
    if get_service_state(service.as_str())?.to_lowercase() != "deactivated" {
        return Err(DabError::Err500(format!("Failed to deactivate service '{}' after 200ms.", service)));
    }
    Ok(())
}

// Parameters: service: The service to check the state of.
// Returns the state of the service:"unavailable/deactivated/deactivation/activated/activation/precondition/hibernated/destroyed"
// on success else DabError.
pub fn get_service_state(service: &str) -> Result<String, DabError> {
    let method = format!("Controller.1.status@{service}");
    let response = rdk_request::<serde_json::Value>(&method)?;
    let state = response["result"][0]["state"]
        .as_str()
        .ok_or(DabError::Err500(format!("Key 'state' not found in response for method '{}'.", method)))?;
    Ok(state.to_string().to_lowercase().clone())
}

// Parameters: service: The service to check the availability of.
// Returns true if the service is available else false on success else DabError.
pub fn service_is_available(service: &str) -> Result<bool, DabError> {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Status {
        autostart: bool,
        callsign: String,
    }

    match rdk_request::<RdkResponse<Vec<Status>>>(format!("Controller.1.status@{service}").as_str())
    {
        Err(message) => {
            let error_message = match &message {
                DabError::Err400(msg) => msg,
                DabError::Err500(msg) => msg,
                DabError::Err501(msg) => msg,
            };

            if error_message == "ERROR_UNKNOWN_KEY" {
                return Ok(false);
            }
            return Err(message);
        }
        Ok(_) => return Ok(true),
    }
}

lazy_static! {
    static ref RDK_KEYMAP: HashMap<String, u16> = {
        let mut keycode_map = HashMap::new();
        keycode_map.insert(String::from("KEY_POWER"),116);
        keycode_map.insert(String::from("KEY_HOME"),36);
        keycode_map.insert(String::from("KEY_VOLUME_UP"),175);
        keycode_map.insert(String::from("KEY_VOLUME_DOWN"),174);
        keycode_map.insert(String::from("KEY_MUTE"),173);
        keycode_map.insert(String::from("KEY_EXIT"),27);
        keycode_map.insert(String::from("KEY_UP"),38);
        keycode_map.insert(String::from("KEY_PAGE_UP"),33);
        keycode_map.insert(String::from("KEY_PAGE_DOWN"),34);
        keycode_map.insert(String::from("KEY_RIGHT"),39);
        keycode_map.insert(String::from("KEY_DOWN"),40);
        keycode_map.insert(String::from("KEY_LEFT"),37);
        keycode_map.insert(String::from("KEY_ENTER"),13);
        keycode_map.insert(String::from("KEY_BACK"),8);
        keycode_map.insert(String::from("KEY_PLAY"),179);
        keycode_map.insert(String::from("KEY_PLAY_PAUSE"),179);
        keycode_map.insert(String::from("KEY_PAUSE"),179);
        keycode_map.insert(String::from("KEY_STOP"),178);
        keycode_map.insert(String::from("KEY_REWIND"),227);
        keycode_map.insert(String::from("KEY_FAST_FORWARD"),228);
        keycode_map.insert(String::from("KEY_SKIP_REWIND"),177);
        keycode_map.insert(String::from("KEY_SKIP_FAST_FORWARD"),176);
        keycode_map.insert(String::from("KEY_0"),48);
        keycode_map.insert(String::from("KEY_1"),49);
        keycode_map.insert(String::from("KEY_2"),50);
        keycode_map.insert(String::from("KEY_3"),51);
        keycode_map.insert(String::from("KEY_4"),52);
        keycode_map.insert(String::from("KEY_5"),53);
        keycode_map.insert(String::from("KEY_6"),54);
        keycode_map.insert(String::from("KEY_7"),55);
        keycode_map.insert(String::from("KEY_8"),56);
        keycode_map.insert(String::from("KEY_9"),57);

        if let Ok(json_file) = read_platform_config_json("/opt/dab_platform_keymap.json") {
            // Platform specific keymap file present in the device
            // Json file should be in below format
            /*
                {
                    "KEY_CHANNEL_UP": 104,
                    "KEY_CHANNEL_DOWN": 109,
                    "KEY_MENU": 408,
                    "KEY_CHANNEL_UP":0,
                    "KEY_CHANNEL_DOWN": 0,
                    "KEY_MENU": 0,
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
            if let Ok(new_keymap) = serde_json::from_str::<HashMap<String, u16>>(&json_file) {
                println!("Imported platform specified keymap /opt/dab_platform_keymap.json.");
                for (key, value) in new_keymap {
                    keycode_map.insert(key, value);
                }
            }
        }
        keycode_map
    };
}

// Static device info; no need to panic or break runtime. Implementation is based on the assumption
// that platform response will be constant for a specific build.
lazy_static! {
    static ref RDK_DEVICE_INFO: HashMap<String, String> = {
        let mut rdk_device_info = HashMap::new();
        match get_thunder_property("DeviceInfo.make", "make") {
            Ok(make) => { rdk_device_info.insert(String::from("manufacturer"), String::from(make)); },
            Err(_err) => {
                if cfg!(debug_assertions) {
                    rdk_device_info.insert(String::from("manufacturer"), String::from("Unknown-manufacturer"));
                }
            },
        };
        match get_thunder_property("DeviceInfo.modelid", "sku") {
            Ok(model) => { rdk_device_info.insert(String::from("model"), String::from(model)); },
            Err(_err) => { 
                if cfg!(debug_assertions) {
                    rdk_device_info.insert(String::from("model"), String::from("Unknown-model"));
                }
            },
        };
        match get_thunder_property("DeviceInfo.serialnumber", "serialnumber") {
            Ok(serialnumber) => { rdk_device_info.insert(String::from("serialnumber"), String::from(serialnumber)); },
            Err(_err) => {
                if cfg!(debug_assertions) {
                    rdk_device_info.insert(String::from("serialnumber"), String::from("Unknown-serialnumber"));
                }
            },
        };
        match get_thunder_property("DeviceIdentification.deviceidentification", "chipset") {
            Ok(chipset) => { rdk_device_info.insert(String::from("chipset"), String::from(chipset)); },
            Err(_err) => {
                if cfg!(debug_assertions) {
                    rdk_device_info.insert(String::from("chipset"), String::from("Unknown-chipset"));
                }
            },
        };
        match get_thunder_property("DeviceInfo.firmwareversion", "imagename") {
            Ok(firmwareversion) => { rdk_device_info.insert(String::from("firmwareversion"), String::from(firmwareversion)); },
            Err(_err) => {
                if cfg!(debug_assertions) {
                    rdk_device_info.insert(String::from("firmwareversion"), String::from("Unknown-FWVersion"));
                }
            },
        };
        rdk_device_info
    };
}

// Parameter: propertyname: The property to get the value of.
// Returns the value of the property on success else DabError.
pub fn get_rdk_device_info(propertyname: &str) -> Result<String, DabError> {
    match RDK_DEVICE_INFO.get(propertyname) {
        Some(val) => Ok(val.clone()),
        None => {
            let error_message = DabError::Err500(format!("No match for property {propertyname}."));
            return Err(error_message);
        }
    }
}

pub fn get_ip_address() -> String {
    unsafe { DEVICE_ADDRESS.clone() }
}

pub fn get_rdk_keys() -> Vec<String> {
    RDK_KEYMAP
        .keys()
        .map(|k| k.to_owned().to_string())
        .collect()
}

pub fn get_keycode(keyname: String) -> Option<&'static u16> {
    RDK_KEYMAP.get(&keyname)
}

pub fn rdk_sound_mode_to_dab(mode: &String) -> Option<AudioOutputMode> {
    match mode.as_str() {
        "STEREO" => Some(AudioOutputMode::Stereo),
        "SURROUND" | "DOLBYDIGITAL" | "DOLBYDIGITALPLUS" => Some(AudioOutputMode::MultichannelPcm),
        "PASSTHRU" => Some(AudioOutputMode::PassThrough),
        _ => {
            if mode.starts_with("AUTO") {
                Some(AudioOutputMode::Auto)
            } else {
                None
            }
        }
    }
}

// Telemetry operations

pub fn get_device_memory() -> Result<u32, DabError> {
    // Both properties are in bytes; convert to KB for DAB.
    let free_ram_bytes = get_thunder_property("DeviceInfo.systeminfo", "freeram")?;
    let free_ram_bytes = free_ram_bytes.parse::<u32>()
        .map_err(|_| DabError::Err500("Failed to parse free RAM".to_string()))? / 1024;

    let total_ram_bytes = get_thunder_property("DeviceInfo.systeminfo", "totalram")?;
    let total_ram_bytes = total_ram_bytes.parse::<u32>()
        .map_err(|_| DabError::Err500("Failed to parse total RAM".to_string()))? / 1024;

    Ok(total_ram_bytes - free_ram_bytes)
}

pub fn get_device_cpu() -> Result<u32, DabError> {
    let cpu_usage = get_thunder_property("DeviceInfo.systeminfo", "cpuload")?;
    let cpu_usage = cpu_usage.parse::<u32>()
        .map_err(|_| DabError::Err500("Failed to parse CPU usage".to_string()))?;

    Ok(cpu_usage)
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

// Function to convert value type to string. Supported types are String, Number and Object.
// Parameters: value: The value to convert to string, key_name: The key name of the value.
// Returns the value as string on success else DabError.
fn convert_value_type_to_string(value: &serde_json::Value, key_name: &str) -> Result<String, String> {
    match value {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Object(o) => serde_json::to_string(o).map_err(|_| format!("Failed to serialize object for key '{}'.", key_name)),
        _ => Err(format!("Unsupported type for key '{}' in response.", key_name)),
    }
}

// Function to get thunder property value. Properties are read-only and will always return a valid value on API success.
// Parameters: method_name: The method name to call, key_name: The key to be matched in the response.
// Returns the value of the key as String on success else DabError.
pub fn get_thunder_property(method_name: &str, key_name: &str) -> Result<String, DabError> {
    let json_string = format!("{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"{}\"}}", method_name);
    let response = http_post(json_string)?;
    let response_value: serde_json::Value = serde_json::from_str(&response).map_err(|e| DabError::Err500(format!("Failed to parse response: {}", e)))?;
    let result = response_value.get("result").ok_or(DabError::Err500(format!("Key 'result' not found in response for method '{}'.", method_name)))?;
    if result.is_null() {
        return Err(DabError::Err500(format!("Key 'result' is null in response for method '{}'.", method_name)));
    }
    if key_name.is_empty() {
        return Ok(result.to_string());
    } else {
        let key_value = result.get(key_name).ok_or(DabError::Err500(format!("Key '{}' not found in response for method '{}'.", key_name, method_name)))?;
        convert_value_type_to_string(key_value, key_name).map_err(|e| DabError::Err500(e))
    }
}

// ############################### APP Lifecycle Time Configs ###############################

type TimeoutMap = HashMap<String, u64>;
type LifecycleTimeouts = HashMap<String, TimeoutMap>;

lazy_static! {
    static ref APP_LIFECYCLE_TIMEOUTS: LifecycleTimeouts = {
        let mut app_lifecycle_timeouts = LifecycleTimeouts::new();

        app_lifecycle_timeouts.insert("youtube".to_string(), {
            let mut timeouts = TimeoutMap::new();
            timeouts.insert("cold_launch_timeout_ms".to_string(), 6000);
            timeouts.insert("resume_launch_timeout_ms".to_string(), 3000);
            timeouts.insert("exit_to_destroy_timeout_ms".to_string(), 2500);
            timeouts.insert("exit_to_background_timeout_ms".to_string(), 2000);
            timeouts
        });

        match read_platform_config_json("/opt/dab_platform_app_lifecycle.json") {
            Ok(json_file) => {
                match serde_json::from_str::<HashMap<String, HashMap<String, u64>>>(&json_file) {
                    Ok(app_lifecycle_config) => {
                        for (app_id, timeout_map) in app_lifecycle_config {
                            if app_id == "youtube" || app_id == "uk.co.bbc.iplayer" || app_id == "netflix" || app_id == "primevideo" {
                                app_lifecycle_timeouts.insert(app_id, timeout_map);
                            }
                        }
                        println!("Imported platform specified app lifetime configuration file also.");
                    }
                    Err(e) => {
                        println!("Failed to parse JSON: {} from 'dab_platform_app_lifecycle.json'.", e);
                    }
                }
            }
            Err(_) => {
                println!("Using default values for app lifecycle timeouts.");
            }
        }

        app_lifecycle_timeouts
    };
}

// Function to get lifecycle timeout for an app. After plugin state change how long App implementation/SDK takes to complete the action.
// Parameters: app_name: The app name (lowercase) to get the timeout for, timeout_type: The type of timeout to get.
// Returns the timeout in milliseconds on success else default 2500.
pub fn get_lifecycle_timeout(app_name: &str, timeout_type: &str) -> Option<u64> {
    APP_LIFECYCLE_TIMEOUTS
        .get(app_name)
        .and_then(|timeouts| timeouts.get(timeout_type))
        .cloned()
        .or_else(|| Some(2500))
}
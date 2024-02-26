use crate::dab::structs::{AudioOutputMode, ErrorResponse};
use futures::executor::block_on;
use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use surf::Client;
static mut DEVICE_ADDRESS: String = String::new();
static mut DEBUG: bool = false;

pub fn init(device_ip: &str, debug: bool) {
    unsafe {
        DEVICE_ADDRESS.push_str(&device_ip);
        DEBUG = debug;
    }
}

pub fn get_device_id() -> String {
    // Update the static details. Nothing to do with the response.
    match get_rdk_device_info(String::from("model")) {
        Some(_) => (),
        None => (),
    }

    let json_string =
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"org.rdk.System.getDeviceInfo\"}".to_string();
    let response = http_post(json_string);
    match response {
        Ok(r) => {
            let response: serde_json::Value = serde_json::from_str(&r).unwrap();
            let device_id = response["result"]["estb_mac"].as_str().unwrap();
            let dab_device_id = device_id.replace(":", "").to_string();
            return dab_device_id;
        }
        Err(err) => {
            return err.to_string();
        }
    }
}

pub fn http_download(url: String) -> Result<(), String> {
    let client = Client::new();

    let response = block_on(async { client.get(url).await });

    match response {
        Ok(mut r) => {
            let mut file = File::create("/tmp/tts.wav").unwrap();
            let body = block_on(r.body_bytes()).unwrap();
            file.write_all(&body).unwrap();
            return Ok(());
        }
        Err(err) => return Err(err.to_string()),
    }
}

pub fn http_post(json_string: String) -> Result<String, String> {
    let client = Client::new();
    let rdk_address = format!("http://{}:9998/jsonrpc", unsafe { &DEVICE_ADDRESS });

    unsafe {
        if DEBUG {
            println!("RDK request: {}", json_string);
        }
    }

    let response = block_on(async {
        client
            .post(rdk_address)
            .body_string(json_string)
            .header("Content-Type", "application/json")
            .await
            .unwrap()
            .body_string()
            .await
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

            return Err(str);
        }
    }
}

pub fn service_deactivate(service: String) -> Result<(), String> {
    //#########Controller.1.deactivate#########
    #[derive(Serialize)]
    struct ControllerDeactivateRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: ControllerDeactivateRequestParams,
    }

    #[derive(Serialize)]
    struct ControllerDeactivateRequestParams {
        callsign: String,
    }

    let req_params = ControllerDeactivateRequestParams { callsign: service };

    let request = ControllerDeactivateRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "Controller.1.deactivate".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct ControllerDeactivateResult {}

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string.clone());

    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(_) => return Ok(()),
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

pub fn rdk_request<R: DeserializeOwned>(method: &str) -> Result<R, String> {
    #[derive(Serialize)]
    struct RdkNullParams {}

    rdk_request_impl::<RdkNullParams,R>(method, None)
}

pub fn rdk_request_with_params<P: Serialize, R: DeserializeOwned>(
    method: &str,
    params: P,
) -> Result<R, String> {
    rdk_request_impl(method, Some(params))
}

fn rdk_request_impl<P: Serialize, R: DeserializeOwned>(
    method: &str,
    params: Option<P>,
) -> Result<R, String> {
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
    let response_json = http_post(json_string)?;

    let val: serde_json::Value = match serde_json::from_str(&response_json) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };

    if val["error"] != serde_json::Value::Null {
        return Err(val["error"]["message"].as_str().unwrap().into());
    } else if !val["result"].is_null() && val["result"]["success"].is_boolean() {
        if !val["result"]["success"].as_bool().unwrap() {
            return Err(format!("{} failed", method));
        }
    }

    let res: R = match serde_json::from_value(val) {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };

    Ok(res)
}

pub fn service_activate(service: String) -> Result<(), String> {
    //#########Controller.1.activate#########
    #[derive(Serialize)]
    struct ControllerActivateRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: ControllerActivateRequestParams,
    }

    #[derive(Serialize)]
    struct ControllerActivateRequestParams {
        callsign: String,
    }

    let req_params = ControllerActivateRequestParams { callsign: service };

    let request = ControllerActivateRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "Controller.1.activate".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct ControllerActivateResult {}

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string.clone());

    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(_) => return Ok(()),
    }
}

pub fn service_is_available(service: &str) -> Result<bool, String> {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Status {
        autostart: bool,
        callsign: String,
    }

    match rdk_request::<RdkResponse<Vec<Status>>> (format!("Controller.1.status@{service}").as_str()) {
        Err(message) => {
            if message == "ERROR_UNKNOWN_KEY" {
                return Ok(false);
            }
            return Err(message);
        }
        Ok(_) => return Ok(true)
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
        // keycode_map.insert(String::from("KEY_CHANNEL_UP"),0);
        // keycode_map.insert(String::from("KEY_CHANNEL_DOWN"),0);
        // keycode_map.insert(String::from("KEY_MENU"),0);
        keycode_map.insert(String::from("KEY_EXIT"),27);
        // keycode_map.insert(String::from("KEY_INFO"),0);
        // keycode_map.insert(String::from("KEY_GUIDE"),0);
        // keycode_map.insert(String::from("KEY_CAPTIONS"),0);
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
        // keycode_map.insert(String::from("KEY_RECORD"),0);
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
        // keycode_map.insert(String::from("KEY_RED"),0);
        // keycode_map.insert(String::from("KEY_GREEN"),0);
        // keycode_map.insert(String::from("KEY_YELLOW"),0);
        // keycode_map.insert(String::from("KEY_BLUE"),0);

        if let Ok(json_file) = read_keymap_json("/opt/dab_platform_keymap.json") {
        // Platform specific keymap file present in the device
        // Json file should be in below format
        // {
        //     "KEY_CHANNEL_UP":104,
        //     "KEY_CHANNEL_DOWN":109,
        //     "KEY_MENU":408
        // }
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

lazy_static! {
    static ref RDK_DEVICE_INFO: HashMap<String, String> = {
        let mut rdk_device_info = HashMap::new();
        match get_thunder_property("DeviceInfo.make", "make") {
            Ok(make) => { rdk_device_info.insert(String::from("manufacturer"), String::from(make)); },
            Err(err) => {
                println!("Error RDK_DEVICE_INFO 'DeviceInfo.make' {}", err);
            },
        };
        match get_thunder_property("DeviceInfo.modelid", "sku") {
            Ok(model) => { rdk_device_info.insert(String::from("model"), String::from(model)); },
            Err(err) => {
                println!("Error RDK_DEVICE_INFO 'DeviceInfo.modelid' {}", err);
            },
        };
        match get_thunder_property("DeviceInfo.serialnumber", "serialnumber") {
            Ok(serialnumber) => { rdk_device_info.insert(String::from("serialnumber"), String::from(serialnumber)); },
            Err(err) => {
                println!("Error RDK_DEVICE_INFO 'DeviceInfo.serialnumber' {}", err);
            },
        };
        match get_thunder_property("DeviceIdentification.deviceidentification", "chipset") {
            Ok(chipset) => { rdk_device_info.insert(String::from("chipset"), String::from(chipset)); },
            Err(err) => {
                println!("Error RDK_DEVICE_INFO 'DeviceIdentification.deviceidentification' {}", err);
            },
        };
        match get_thunder_property("DeviceInfo.firmwareversion", "imagename") {
            Ok(firmwareversion) => { rdk_device_info.insert(String::from("firmwareversion"), String::from(firmwareversion)); },
            Err(err) => {
                println!("Error RDK_DEVICE_INFO 'DeviceInfo.firmwareversion' {}", err);
            },
        };
        rdk_device_info
    };
}

pub fn get_rdk_device_info(propertyname: String) -> Option<&'static String> {
    RDK_DEVICE_INFO.get(&propertyname)
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

pub fn get_device_memory() -> Result<u32, String> {
    Ok(0)
}

//Read key inputs from file
// This is optional override configuration. Do not return error.
pub fn read_keymap_json(file_path: &str) -> Result<String, String> {
    let mut file_content = String::new();
    File::open(file_path)
        .map_err(|e| {
            if e.kind() != std::io::ErrorKind::NotFound {
                println!("Error opening {}: {}", file_path, e);
            }
            e.to_string()
        })?
        .read_to_string(&mut file_content)
        .map_err(|e| {
            println!("Error reading {}: {}", file_path, e);
            e.to_string()
        })?;
    Ok(file_content)
}

fn convert_value_type_to_string(value: &serde_json::Value, key_name: &str) -> Result<String, String> {
    match value {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Object(o) => serde_json::to_string(o).map_err(|_| format!("Failed to serialize object for key '{}'.", key_name)),
        _ => Err(format!("Unsupported type for key '{}' in response.", key_name)),
    }
}

// Function to get thunder property value. Properties are read-only and will always return a valid value if not error.
// If the key is not found in the response, it will return a dummy response in debug mode.
pub fn get_thunder_property(method_name: &str, key_name: &str) -> Result<String, String> {
    let json_string = format!("{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"{}\"}}", method_name);
    let response = http_post(json_string).map_err(|err| {
        let error = ErrorResponse {
            status: 500,
            error: err,
        };
        serde_json::to_string(&error).unwrap_or_else(|_| "Failed to serialize error.".to_string())
    })?;

    let response: serde_json::Value = serde_json::from_str(&response).map_err(|_| "Failed to parse response.".to_string())?;
    let value = if key_name.is_empty() {
        &response["result"]
    } else {
        &response["result"][key_name]
    };

    if value.is_null() {
        if cfg!(debug_assertions) {
            println!("Key '{}' not found in response.", key_name);
            Ok(format!("Dummy response for {}.", key_name))
        } else {
            Err(format!("Key '{}' not found in response.", key_name))
        }
    } else {
        convert_value_type_to_string(value, key_name)
    }
}

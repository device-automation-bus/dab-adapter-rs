use crate::dab::structs::AudioOutputMode;
use crate::dab::structs::DabError;
use futures::executor::block_on;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
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
}

pub fn get_device_id() -> Result<String, DabError> {
    let json_string =
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"org.rdk.System.getDeviceInfo\"}".to_string();
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

pub fn service_deactivate(service: String) -> Result<(), DabError> {
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
    http_post(json_string)?;
    Ok(())
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

pub fn service_activate(service: String) -> Result<(), DabError> {
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
    http_post(json_string)?;
    Ok(())
}

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
    Ok(0)
}

//Read key inputs from file

pub fn read_keymap_json(file_path: &str) -> Result<String, DabError> {
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

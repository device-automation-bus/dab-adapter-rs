use crate::dab::ErrorResponse;
use crate::device::rdk::output::image::save_image;
use futures::executor::block_on;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use surf::Client;
use tokio;
static mut DEVICE_ADDRESS: String = String::new();

pub async fn init(device_ip: &str) {
    unsafe {
        DEVICE_ADDRESS.push_str(&device_ip);
    }
    tokio::spawn(start_http_server());
}

async fn start_http_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use hyper::server::Server;
    use hyper::service::{make_service_fn, service_fn};

    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(save_image)) });

    let addr = ([0, 0, 0, 0], 7878).into();
    let http_server = Server::bind(&addr).serve(make_service);
    println!("Started http server at port 7878 for dab/output/image operator");
    http_server.await?;
    Ok(())
}

pub fn get_device_id() -> String {
    let json_string =
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"org.rdk.System.getDeviceInfo\"}"
            .to_string();
    let response = http_post(json_string);
    match response {
        Ok(r) => {
            let response: serde_json::Value = serde_json::from_str(&r).unwrap();
            let device_id = response["result"]["estb_mac"].as_str().unwrap();
            let dab_device_id = device_id.replace(":", "").to_string();
            println!("DAB Device ID: {}", dab_device_id);
            return dab_device_id;
        }
        Err(err) => {
            return err.to_string();
        }
    }
}

pub fn upload_image(url: String) -> Result<(), String> {
    // Read the TIFF image file into a Vec<u8>
    let filepath = "/tmp/screenshot.tiff";
    let mut file = File::open(filepath).unwrap();
    let mut buffer = Vec::new();
    let result = file.read_to_end(&mut buffer);
    match result {
        Ok(_) => {}
        Err(err) => {
            return Err(err.to_string());
        }
    }

    // Create a surf::Client
    let client = Client::new();

    // Create a PUT request
    let response = block_on(async {
        client
            .put(url)
            .body_bytes(&buffer)
            .header("Content-Type", "image/tiff")
            .await
            .unwrap()
            .body_string()
            .await
    });

    match response {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn http_download(url: String) -> Result<(), String> {
    let client = Client::new();

    let response = block_on(async { client.get(url).await });

    match response {
        Ok(mut r) => {
            let mut file = File::create("/tmp/tts.wav").unwrap();
            let body = block_on(async { r.body_bytes().await.unwrap() });
            file.write_all(&body).unwrap();
            return Ok(());
        }
        Err(err) => return Err(err.to_string()),
    }
}

pub fn http_post(json_string: String) -> Result<String, String> {
    let client = Client::new();
    let rdk_address = format!("http://{}:9998/jsonrpc", unsafe { &DEVICE_ADDRESS });

    println!("JSON string is {}", json_string);
    println!("rdk_address is : {}", rdk_address);

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
            println!("OK response is: {}", r.to_string());
            return Ok(r.to_string());
        }
        Err(err) => {
            println!("Err response is: {}", err.to_string());
            return Err(err.to_string());
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

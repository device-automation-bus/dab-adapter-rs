use crate::dab::structs::{AudioOutputMode, DabError};
use crate::hw_specific::interface::http::http_post;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::{thread, time};


static JSONRPC_ID: AtomicI32 = AtomicI32::new(1);

fn next_id() -> i32 {
    JSONRPC_ID.fetch_add(1, Ordering::Relaxed)
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

    let id = next_id();

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
    })
    .to_string();
    let response = http_post(activate_payload)?;
    let response_value: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| DabError::Err500(format!("Failed to parse response: {}", e)))?;
    if response_value.get("result").is_none() {
        return Err(DabError::Err500(format!(
            "Key 'result' not found in response for method 'Controller.1.activate'."
        )));
    }
    thread::sleep(time::Duration::from_millis(200));
    if get_service_state(service.as_str())?.to_lowercase() != "activated" {
        return Err(DabError::Err500(format!(
            "Failed to activate service '{}' after 200ms.",
            service
        )));
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
    })
    .to_string();
    let response = http_post(activate_payload)?;
    let response_value: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| DabError::Err500(format!("Failed to parse response: {}", e)))?;
    if response_value.get("result").is_none() {
        return Err(DabError::Err500(format!(
            "Key 'result' not found in response for method 'Controller.1.activate'."
        )));
    }
    thread::sleep(time::Duration::from_millis(200));
    if get_service_state(service.as_str())?.to_lowercase() != "deactivated" {
        return Err(DabError::Err500(format!(
            "Failed to deactivate service '{}' after 200ms.",
            service
        )));
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
        .ok_or(DabError::Err500(format!(
            "Key 'state' not found in response for method '{}'.",
            method
        )))?;
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

// Function to get thunder property value. Properties are read-only and will always return a valid value on API success.
// Parameters: method_name: The method name to call, key_name: The key to be matched in the response.
// Returns the value of the key as String on success else DabError.
pub fn get_thunder_property(method_name: &str, key_name: &str) -> Result<String, DabError> {
    let json_string = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"{}\"}}",
        method_name
    );
    let response = http_post(json_string)?;
    let response_value: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| DabError::Err500(format!("Failed to parse response: {}", e)))?;
    let result = response_value
        .get("result")
        .ok_or(DabError::Err500(format!(
            "Key 'result' not found in response for method '{}'.",
            method_name
        )))?;
    if result.is_null() {
        return Err(DabError::Err500(format!(
            "Key 'result' is null in response for method '{}'.",
            method_name
        )));
    }

    let value = if !key_name.is_empty() {
        result.get(key_name).ok_or(DabError::Err500(format!(
            "Key '{}' not found in response for method '{}'.",
            key_name, method_name
        )))?
    } else {
        result
    };

    match value {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Object(o) => serde_json::to_string(o).map_err(|_| {
            DabError::Err500(format!(
                "Failed to serialize object for key '{}'.",
                key_name
            ))
        }),
        _ => Err(DabError::Err500(format!(
            "Unsupported type for key '{}' in response.",
            key_name
        ))),
    }
}

// Static device info; no need to panic or break runtime. Implementation is based on the assumption
// that platform response will be constant for a specific build.
pub fn get_device_info() -> HashMap<String, String> {
    let mut rdk_device_info = HashMap::new();
    match get_thunder_property("DeviceInfo.make", "make") {
        Ok(make) => {
            rdk_device_info.insert(String::from("manufacturer"), String::from(make));
        }
        Err(_err) => {
            if cfg!(debug_assertions) {
                rdk_device_info.insert(
                    String::from("manufacturer"),
                    String::from("Unknown-manufacturer"),
                );
            }
        }
    };
    match get_thunder_property("DeviceInfo.modelid", "sku") {
        Ok(model) => {
            rdk_device_info.insert(String::from("model"), String::from(model));
        }
        Err(_err) => {
            if cfg!(debug_assertions) {
                rdk_device_info.insert(String::from("model"), String::from("Unknown-model"));
            }
        }
    };
    match get_thunder_property("DeviceInfo.serialnumber", "serialnumber") {
        Ok(serialnumber) => {
            rdk_device_info.insert(String::from("serialnumber"), String::from(serialnumber));
        }
        Err(_err) => {
            if cfg!(debug_assertions) {
                rdk_device_info.insert(
                    String::from("serialnumber"),
                    String::from("Unknown-serialnumber"),
                );
            }
        }
    };
    match get_thunder_property("DeviceInfo.socname", "socname") {
        Ok(socname) => {
            rdk_device_info.insert(String::from("chipset"), String::from(socname));
        }
        Err(_err) => {
            eprintln!(
                "Unable to retrieve chipset from DeviceInfo, trying legacy DeviceIdentification."
            );
            match get_thunder_property("DeviceIdentification.deviceidentification", "chipset") {
                Ok(chipset) => {
                    rdk_device_info.insert(String::from("chipset"), String::from(chipset));
                }
                Err(_err) => {
                    if cfg!(debug_assertions) {
                        rdk_device_info
                            .insert(String::from("chipset"), String::from("Unknown-chipset"));
                    }
                }
            };
        }
    };
    match get_thunder_property("DeviceInfo.firmwareversion", "imagename") {
        Ok(firmwareversion) => {
            rdk_device_info.insert(
                String::from("firmwareversion"),
                String::from(firmwareversion),
            );
        }
        Err(_err) => {
            if cfg!(debug_assertions) {
                rdk_device_info.insert(
                    String::from("firmwareversion"),
                    String::from("Unknown-FWVersion"),
                );
            }
        }
    };
    rdk_device_info
}

pub fn get_rdk_device_id() -> Result<String, DabError> {
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

pub fn get_device_memory() -> Result<u32, DabError> {
    // Both properties are in bytes; convert to KB for DAB.
    let free_ram_bytes = get_thunder_property("DeviceInfo.systeminfo", "freeram")?;
    let free_ram_bytes = free_ram_bytes
        .parse::<u32>()
        .map_err(|_| DabError::Err500("Failed to parse free RAM".to_string()))?
        / 1024;

    let total_ram_bytes = get_thunder_property("DeviceInfo.systeminfo", "totalram")?;
    let total_ram_bytes = total_ram_bytes
        .parse::<u32>()
        .map_err(|_| DabError::Err500("Failed to parse total RAM".to_string()))?
        / 1024;

    Ok(total_ram_bytes - free_ram_bytes)
}

pub fn get_device_cpu() -> Result<u32, DabError> {
    let cpu_usage = get_thunder_property("DeviceInfo.systeminfo", "cpuload")?;
    let cpu_usage = cpu_usage
        .parse::<u32>()
        .map_err(|_| DabError::Err500("Failed to parse CPU usage".to_string()))?;

    Ok(cpu_usage)
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

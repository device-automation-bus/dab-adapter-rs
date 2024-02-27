// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct DeviceInfoRequest {}

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum NetworkInterfaceType{#[default]
//    Ethernet,
//    Wifi,
//    Bluetooth,
//    Coax,
//    Other,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum DisplayType{#[default]
//    Native,
//    External,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct NetworkInterface{
// pub connected: bool,
// pub macAddress: String,
// pub r#type: NetworkInterfaceType,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct DeviceInformation{
// pub manufacturer: String,
// pub model: String,
// pub serialNumber: String,
// pub chipset: String,
// pub firmwareVersion: String,
// pub firmwareBuild: String,
// pub networkInterfaces: Vec<NetworkInterface>,
// pub displayType: DisplayType,
// pub screenWidthPixels: u32,
// pub screenHeightPixels: u32,
// pub uptimeSince: u64,
// }

use std::time::{SystemTime, UNIX_EPOCH};
#[allow(unused_imports)]
use crate::dab::structs::DeviceInfoRequest;
use crate::dab::structs::DisplayType;
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::GetDeviceInformationResponse;
#[allow(unused_imports)]
use crate::dab::structs::NetworkInterface;
use crate::dab::structs::NetworkInterfaceType;
use crate::device::rdk::interface::get_device_id;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::{get_rdk_device_info, get_thunder_property};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = GetDeviceInformationResponse::default();
    // *** Fill in the fields of the struct DeviceInformation here ***

    //#########org.rdk.DisplaySettings.getConnectedVideoDisplays#########
    #[derive(Serialize)]
    struct GetConnectedVideoDisplaysRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetConnectedVideoDisplaysRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.DisplaySettings.getConnectedVideoDisplays".into(),
    };

    #[derive(Deserialize)]
    struct GetConnectedVideoDisplaysResponse {
        jsonrpc: String,
        id: i32,
        result: GetConnectedVideoDisplaysResult,
    }

    #[derive(Deserialize)]
    struct GetConnectedVideoDisplaysResult {
        success: bool,
        connectedVideoDisplays: Vec<String>,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    let ConnectedVideoDisplays: GetConnectedVideoDisplaysResponse;
    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(response) => {
            ConnectedVideoDisplays = serde_json::from_str(&response).unwrap();
        }
    }

    //######### Map from Static Hashmap: Begin #########

    ResponseOperator.manufacturer = get_rdk_device_info("manufacturer")?;
    ResponseOperator.model = get_rdk_device_info("model")?;
    ResponseOperator.serialNumber = get_rdk_device_info("serialnumber")?;
    ResponseOperator.chipset = get_rdk_device_info("chipset")?;
    // Both firmwareVersion and firmwareBuild are same for RDKV devices.
    ResponseOperator.firmwareVersion = get_rdk_device_info("firmwareversion")?;
    ResponseOperator.firmwareBuild = get_rdk_device_info("firmwareversion")?;
    
    //######### Map from Static Hashmap: End #########

    //#########org.rdk.RDKShell.getScreenResolution#########
    #[derive(Serialize)]
    struct GetScreenResolutionRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetScreenResolutionRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.getScreenResolution".into(),
    };

    #[derive(Deserialize)]
    struct GetScreenResolutionResponse {
        jsonrpc: String,
        id: i32,
        result: GetScreenResolutionResult,
    }

    #[derive(Deserialize)]
    struct GetScreenResolutionResult {
        w: u32,
        h: u32,
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    let ScreenResolution: GetScreenResolutionResponse;
    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(response) => {
            ScreenResolution = serde_json::from_str(&response).unwrap();
        }
    }

    //#########org.rdk.Network.getInterfaces#########
    #[derive(Serialize)]
    struct GetInterfacesRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetInterfacesRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.Network.getInterfaces".into(),
    };

    #[derive(Deserialize)]
    struct Interface {
        interface: String,
        macAddress: String,
        enabled: bool,
        connected: bool,
    }

    #[derive(Deserialize)]
    struct GetInterfacesResult {
        interfaces: Vec<Interface>,
    }

    #[derive(Deserialize)]
    struct GetInterfacesResponse {
        jsonrpc: String,
        id: i32,
        result: GetInterfacesResult,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    let mut Interfaces: GetInterfacesResponse;
    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(response) => {
            Interfaces = serde_json::from_str(&response).unwrap();
        }
    }

    //#########DeviceInfo.systeminfo#########

    let mut device_uptime: u64 = match get_thunder_property("DeviceInfo.systeminfo","uptime") {
        Ok(uptime) => uptime.parse::<u64>().unwrap_or(0),
        Err(_) => {
            // Use from '/proc/uptime' if 'DeviceInfo.systeminfo' is not available
            if let Ok(contents) = std::fs::read_to_string("/proc/uptime") {
                if let Some(uptime_str) = contents.split_whitespace().next() {
                    if let Ok(uptime) = uptime_str.parse::<f64>() {
                        uptime as u64
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            }
        },
    };

    //######### Correlate Fields #########

    for iface in Interfaces.result.interfaces.iter_mut() {
        let mut interface = NetworkInterface {
            r#type: NetworkInterfaceType::Other,
            ..Default::default()
        };

        match iface.interface.clone().as_str() {
            "ETHERNET" => interface.r#type = NetworkInterfaceType::Ethernet,
            "WIFI" => interface.r#type = NetworkInterfaceType::Wifi,
            _ => interface.r#type = NetworkInterfaceType::Other,
        }

        interface.connected = iface.connected;
        interface.macAddress = iface.macAddress.clone();
        // #########org.rdk.Network.getIPSettings#########

        #[derive(Serialize)]
        struct GetIPSettingsRequest {
            jsonrpc: String,
            id: i32,
            method: String,
            params: GetIPSettingsRequestParams,
        }

        #[derive(Serialize)]
        struct GetIPSettingsRequestParams {
            interface: String,
        }

        let req_params = GetIPSettingsRequestParams {
            interface: iface.interface.clone(),
        };

        let request = GetIPSettingsRequest {
            jsonrpc: "2.0".into(),
            id: 3,
            method: "org.rdk.Network.getIPSettings".into(),
            params: req_params,
        };

        #[derive(Deserialize)]
        struct GetIPSettingsResponse {
            jsonrpc: String,
            id: i32,
            result: GetIPSettingsResult,
        }

        #[derive(Deserialize)]
        struct GetIPSettingsResult {
            pub interface: Option<String>,
            pub ipversion: Option<String>,
            pub autoconfig: Option<bool>,
            pub ipaddr: Option<String>, // maps to `ipAddress`
            pub netmask: Option<String>,
            pub gateway: Option<String>,
            pub primarydns: Option<String>,
            pub secondarydns: Option<String>,
            pub success: bool,
        }

        let json_string = serde_json::to_string(&request).unwrap();
        let response_json = http_post(json_string);

        match response_json {
            Err(err) => {
                let error = ErrorResponse {
                    status: 500,
                    error: err,
                };
                return Err(serde_json::to_string(&error).unwrap());
            }
            Ok(response) => {
                let IPSettings: GetIPSettingsResponse = serde_json::from_str(&response).unwrap();
                if let Some(ipaddr) = IPSettings.result.ipaddr {
                    interface.ipAddress = ipaddr;
                }

                for dnsparam in [ IPSettings.result.primarydns, IPSettings.result.secondarydns ] {
                    if let Some(dns) = dnsparam {
                        if !dns.is_empty() {
                            interface.dns.push(dns)
                        }
                    }
                }
            }
        }
        ResponseOperator.networkInterfaces.push(interface);
    }

    let ms_since_epoch = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| {
            serde_json::to_string(&ErrorResponse { status: 500, error: err.to_string() }).unwrap()
        })?
        .as_secs() - device_uptime) * 1000;

    ResponseOperator.uptimeSince = ms_since_epoch;
    ResponseOperator.screenWidthPixels = ScreenResolution.result.w;
    ResponseOperator.screenHeightPixels = ScreenResolution.result.h;
    ResponseOperator.deviceId = get_device_id();

    if ConnectedVideoDisplays.result.connectedVideoDisplays[0].contains("HDMI") {
        ResponseOperator.displayType = DisplayType::External;
    } else {
        ResponseOperator.displayType = DisplayType::Native;
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

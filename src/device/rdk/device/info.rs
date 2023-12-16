use std::time::{SystemTime, UNIX_EPOCH};
#[allow(unused_imports)]
use crate::dab::structs::DeviceInfoRequest;
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::DisplayType;
use crate::dab::structs::GetDeviceInformationResponse;
#[allow(unused_imports)]
use crate::dab::structs::NetworkInterface;
use crate::dab::structs::NetworkInterfaceType;
use crate::device::rdk::interface::get_device_id;
use crate::device::rdk::interface::http_post;
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
    let response = http_post(json_string)?;
    let ConnectedVideoDisplays: GetConnectedVideoDisplaysResponse;
    ConnectedVideoDisplays = serde_json::from_str(&response).unwrap();
    //#########org.rdk.System.getDeviceInfo#########
    #[derive(Serialize)]
    struct GetDeviceInfoRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetDeviceInfoRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.System.getDeviceInfo".into(),
    };

    #[derive(Deserialize)]
    struct GetDeviceInfoResponse {
        jsonrpc: String,
        id: i32,
        result: GetDeviceInfoResult,
    }

    #[derive(Deserialize)]
    struct GetDeviceInfoResult {
        make: String, // maps to `manufacturer`
        bluetooth_mac: String,
        boxIP: String,
        build_type: String,
        esn: String,
        estb_mac: String,
        eth_mac: String,
        friendly_id: String,
        imageRevision: String, // maps to `firmwareVersion`
        imageVersion: String,  // maps to `firmwareBuild`
        version: String,
        software_version: String,
        model_number: String, // maps to `model`
        wifi_mac: String,
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response = http_post(json_string)?;

    let DeviceInfo: GetDeviceInfoResponse;
    DeviceInfo = serde_json::from_str(&response).unwrap();

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
    let response = http_post(json_string)?;

    let ScreenResolution: GetScreenResolutionResponse;
    ScreenResolution = serde_json::from_str(&response).unwrap();

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
    let response = http_post(json_string)?;
    let mut Interfaces: GetInterfacesResponse;
    Interfaces = serde_json::from_str(&response).unwrap();

    //#########DeviceInfo.systeminfo#########
    #[derive(Serialize)]
    struct SysteminfoRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = SysteminfoRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "DeviceInfo.systeminfo".into(),
    };

    #[derive(Deserialize)]
    struct SysteminfoResponse {
        jsonrpc: String,
        id: i32,
        result: SysteminfoResult,
    }

    #[derive(Deserialize)]
    struct SysteminfoResult {
        pub version: String,
        pub uptime: u64, // maps to `uptimeSince`
        pub totalram: u64,
        pub freeram: u64,
        pub devicename: String,
        pub cpuload: String,
        pub serialnumber: String, // maps to `serialNumber`
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response = http_post(json_string)?;
    let Systeminfo: SysteminfoResponse;
    Systeminfo = serde_json::from_str(&response).unwrap();
    
    //#########DeviceIdentification.1.deviceidentification#########

    #[derive(Serialize)]
    struct DeviceidentificationRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    #[derive(Deserialize)]
    struct DeviceidentificationResult {
        pub firmwareversion: String,
        pub chipset: String, // maps to `chipset`
        pub deviceid: String,
    }

    let request = DeviceidentificationRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "DeviceIdentification.1.deviceidentification".into(),
    };

    #[derive(Deserialize)]
    struct DeviceidentificationResponse {
        jsonrpc: String,
        id: i32,
        result: DeviceidentificationResult,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response = http_post(json_string)?;
    let Deviceidentification: DeviceidentificationResponse;
    Deviceidentification = serde_json::from_str(&response).unwrap();
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
        let response = http_post(json_string)?;

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

        ResponseOperator.networkInterfaces.push(interface);
    }

    let ms_since_epoch = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| {
            serde_json::to_string(&ErrorResponse { status: 500, error: err.to_string() }).unwrap()
        })?
        .as_secs() - Systeminfo.result.uptime) * 1000;

    ResponseOperator.serialNumber = Systeminfo.result.serialnumber;
    ResponseOperator.uptimeSince = ms_since_epoch;
    ResponseOperator.manufacturer = DeviceInfo.result.make;
    ResponseOperator.firmwareVersion = DeviceInfo.result.imageRevision;
    ResponseOperator.firmwareBuild = DeviceInfo.result.imageVersion;
    ResponseOperator.model = DeviceInfo.result.model_number;
    ResponseOperator.chipset = Deviceidentification.result.chipset;
    ResponseOperator.screenWidthPixels = ScreenResolution.result.w;
    ResponseOperator.screenHeightPixels = ScreenResolution.result.h;
    ResponseOperator.deviceId = get_device_id()?;

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

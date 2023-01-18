pub mod info {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod rpc {
        use serde::{Deserialize, Serialize};
        use crate::utils;

        // This endpoint needs a lot of different information,
        // we can only get through multiple RPC calls to different
        // methods. Therefore we need different `*Result` and
        // `*Response` structs for each RPC call

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct DeviceInfoResult {
            pub make: Option<String>, // maps to `manufacturer`
            pub bluetooth_mac: Option<String>,
            pub boxIP: Option<String>,
            pub build_type: Option<String>,
            pub esn: Option<String>,
            pub estb_mac: Option<String>,
            pub eth_mac: Option<String>,
            pub friendly_id: Option<String>,
            pub imageRevision: Option<String>, // maps to `firmwareVersion`
            pub imageVersion: Option<String>, // maps to `firmwareBuild`
            pub version: Option<String>,
            pub software_version: Option<String>,
            pub model_number: Option<String>, // maps to `model`
            pub wifi_mac: Option<String>,
            pub success: bool,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct DeviceInfoResponse {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<DeviceInfoResult>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        impl utils::Response for DeviceInfoResponse {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct ConnectedVideoDisplaysResult {
            pub connectedVideoDisplays: Option<Vec<String>>,
            pub success: bool,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct ConnectedVideoDisplaysResponse {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<ConnectedVideoDisplaysResult>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        impl utils::Response for ConnectedVideoDisplaysResponse {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct ScreenResolutionResult {
            pub h: Option<u32>, // maps to `screenHeightPixels`
            pub w: Option<u32>, // maps to `screenWidthPixels`
            pub success: bool,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct ScreenResolutionResponse {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<ScreenResolutionResult>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        impl utils::Response for ScreenResolutionResponse {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct SystemInfoResult {
            pub version: String,
            pub uptime: u64, // maps to `uptimeSince`
            pub totalram: u64,
            pub freeram: u64,
            pub devicename: String,
            pub cpuload: String,
            pub serialnumber: String, // maps to `serialNumber`
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct SystemInfoResponse {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<SystemInfoResult>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct DeviceIdentificationResult {
            pub firmwareversion: String,
            pub chipset: String, // maps to `chipset`
            pub deviceid: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct DeviceIdentificationResponse {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<DeviceIdentificationResult>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Interface {
            pub interface: String, // maps to `type` (upper case, needs to be lowercased first)
            pub macAddress: String, // maps to `macAddress`
            pub enabled: bool,
            pub connected: bool, // maps to `connected`
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct NetworkInterfacesResult {
            pub interfaces: Option<Vec<Interface>>, // maps to `networkinterfaces[]`
            pub success: bool,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct NetworkInterfacesResponse {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<NetworkInterfacesResult>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        impl utils::Response for NetworkInterfacesResponse {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct IPSettingsParams {
            pub interface: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct IPSettingsRequest {
            pub jsonrpc: String,
            pub id: u64,
            pub method: String,
            pub params: IPSettingsParams,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct IPSettingsResult {
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

        #[derive(Serialize, Deserialize, Debug)]
        pub struct IPSettingsResponse {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<IPSettingsResult>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        impl utils::Response for IPSettingsResponse {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }
    }

    mod dab {
        use serde::{Deserialize, Serialize};

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct NetworkInterface {
            pub connected: bool,
            pub macAddress: String,
            pub ipAddress: Option<String>,
            pub r#type: String,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct Response {
            pub status: u16,
            pub manufacturer: String,
            pub model: String,
            pub serialNumber: String,
            pub chipset: String,
            pub firmwareVersion: String,
            pub firmwareBuild: String,
            pub networkInterfaces: Vec<NetworkInterface>,
            pub displayType: String,
            pub screenWidthPixels: u32,
            pub screenHeightPixels: u32,
            pub uptimeSince: u64,
        }
    }

    fn get_device_info(response: &mut dab::Response, ws: &mut utils::WsStream) -> Result<()> {
        let request = utils::rpc::SimpleRequest {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.System.getDeviceInfo".to_string(),
        };

        let mut r = String::new();
        match utils::rpc::call::<utils::rpc::SimpleRequest, rpc::DeviceInfoResponse>(request, &mut r, ws) {
            Ok(r) => {
                let result = r.result.unwrap();
                response.manufacturer = result.make.unwrap_or("unknown".to_string());
                response.model = result.model_number.unwrap_or("unknown".to_string());
                response.firmwareVersion = result.imageRevision.unwrap_or("unknown".to_string());
                response.firmwareBuild = result.imageVersion.unwrap_or("unknown".to_string());
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    fn get_display_info(response: &mut dab::Response, ws: &mut utils::WsStream) -> Result<()> {
        let request = utils::rpc::SimpleRequest {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.DisplaySettings.getConnectedVideoDisplays".to_string(),
        };

        let mut r = String::new();
        match utils::rpc::call::<utils::rpc::SimpleRequest, rpc::ConnectedVideoDisplaysResponse>(request, &mut r, ws) {
            Ok(r) => {
                let result = r.result.unwrap();
                match result.connectedVideoDisplays {
                    Some(video_displays) => {
                        if video_displays[0].contains("HDMI") {
                            response.displayType="Native".to_string();
                        }
                        else{
                            response.displayType="External".to_string();
                        }
                    },
                    None => (),
                }
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    fn get_screen_resolution(response: &mut dab::Response, ws: &mut utils::WsStream) -> Result<()> {
        let request = utils::rpc::SimpleRequest {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.RDKShell.getScreenResolution".to_string(),
        };

        let mut r = String::new();
        match utils::rpc::call::<utils::rpc::SimpleRequest, rpc::ScreenResolutionResponse>(request, &mut r, ws) {
            Ok(r) => {
                let result = r.result.unwrap();
                response.screenWidthPixels = result.w.unwrap_or(0);
                response.screenHeightPixels = result.h.unwrap_or(0);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    fn get_system_info(response: &mut dab::Response, ws: &mut utils::WsStream) -> Result<()> {
        let request = utils::rpc::SimpleRequest {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "DeviceInfo.1.systeminfo".to_string(),
        };

        // The result for this RPC call lacks a `success` field, so we must use
        // `call_raw()` and process the result manually
        match utils::rpc::call_raw::<utils::rpc::SimpleRequest>(request, ws) {
            Ok(r) => {
                match serde_json::from_str::<rpc::SystemInfoResponse>(r.as_str()) {
                    Ok(r) => {
                        let result = r.result.unwrap();
                        response.serialNumber = result.serialnumber;
                        response.uptimeSince = result.uptime;
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    fn get_device_identification(response: &mut dab::Response, ws: &mut utils::WsStream) -> Result<()> {
        let request = utils::rpc::SimpleRequest {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "DeviceIdentification.1.deviceidentification".to_string(),
        };

        // The result for this RPC call lacks a `success` field, so we must use
        // `call_raw()` and process the result manually
        match utils::rpc::call_raw::<utils::rpc::SimpleRequest>(request, ws) {
            Ok(r) => {
                match serde_json::from_str::<rpc::DeviceIdentificationResponse>(r.as_str()) {
                    Ok(r) => {
                        let result = r.result.unwrap();
                        response.chipset = result.chipset;
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e)
        }
    }

    fn get_network_interfaces(response: &mut dab::Response, ws: &mut utils::WsStream) -> Result<()> {
        let request = utils::rpc::SimpleRequest {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.Network.getInterfaces".to_string(),
        };

        let mut s = String::new();
        match utils::rpc::call::<utils::rpc::SimpleRequest, rpc::NetworkInterfacesResponse>(request, &mut s, ws) {
            Ok(r) => {
                let result = r.result.unwrap();
                match result.interfaces {
                    Some(interfaces) => {
                        // Interfaces lack the IP address, so we must make an additional RPC call
                        // for each connected interface in order to get their IP
                        for iface in interfaces {
                            let mut iface_info = dab::NetworkInterface {
                                connected: iface.connected,
                                macAddress: iface.macAddress,
                                ipAddress: None,
                                r#type: iface.interface.to_lowercase(),
                            };

                            if iface.connected {
                                let request = rpc::IPSettingsRequest {
                                    jsonrpc: "2.0".to_string(),
                                    id: utils::get_request_id(),
                                    method: "org.rdk.Network.getIPSettings".to_string(),
                                    params: rpc::IPSettingsParams {
                                        interface: iface.interface,
                                    },
                                };
                                
                                match utils::rpc::call::<rpc::IPSettingsRequest, rpc::IPSettingsResponse>(request, &mut s, ws) {
                                    Ok(r) => {
                                        if let Some(a) = r.result.unwrap().ipaddr {
                                            iface_info.ipAddress = Some(a)
                                        }
                                    },
                                    Err(e) => return Err(e),
                                }
                            }

                            response.networkInterfaces.push(iface_info);
                        }
                    },
                    None => (),
                }
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn process(_packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        let mut dab_response = dab::Response::default();
        if let Err(e) = get_device_info(&mut dab_response, ws) {
            return Err(e);
        }
        if let Err(e) = get_screen_resolution(&mut dab_response, ws) {
            return Err(e);
        }
        if let Err(e) = get_system_info(&mut dab_response, ws) {
            return Err(e);
        }
        if let Err(e) = get_device_identification(&mut dab_response, ws) {
            return Err(e);
        }
        if let Err(e) = get_network_interfaces(&mut dab_response, ws) {
            return Err(e);
        }
        if let Err(e) = get_display_info(&mut dab_response, ws) {
            return Err(e);
        }
        dab_response.status = 200;
        serde_json::to_string(&dab_response)
    }
}

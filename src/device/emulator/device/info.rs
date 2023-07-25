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
// pub ipAddress: String,
// pub dns: Vec<String>,
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

#[allow(unused_imports)]
use crate::dab::structs::DeviceInfoRequest;
use crate::dab::structs::DeviceInformation;
#[allow(unused_imports)]
use crate::dab::structs::NetworkInterface;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = DeviceInformation::default();
    // *** Fill in the fields of the struct DeviceInformation here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct DeviceInfoRequest {}

#[allow(dead_code)]
#[derive(Default,Serialize,Deserialize)]
pub enum NetworkInterfaceType{#[default]
	   Ethernet,
	   Wifi,
	   Bluetooth,
	   Coax,
	   Other,
}

#[allow(dead_code)]
#[derive(Default,Serialize,Deserialize)]
pub enum DisplayType{#[default]
	   Native,
	   External,
}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct NetworkInterface{
	pub connected: bool,
	pub macAddress: String,
	pub r#type: NetworkInterfaceType,
}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct DeviceInformation{
	pub manufacturer: String,
	pub model: String,
	pub serialNumber: String,
	pub chipset: String,
	pub firmwareVersion: String,
	pub firmwareBuild: String,
	pub networkInterfaces: Vec<NetworkInterface>,
	pub displayType: DisplayType,
	pub screenWidthPixels: f32,
	pub screenHeightPixels: f32,
	pub uptimeSince: u32,
}


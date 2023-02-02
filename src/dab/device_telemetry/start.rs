use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct StartDeviceTelemetryRequest{
	pub frequency: f32,
}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct StartDeviceTelemetryResponse{
	pub frequency: f32,
}


use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct SetSystemSettingsRequest{
	pub language: String,
}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct SetSystemSettingsResponse{
	pub language: String,
}


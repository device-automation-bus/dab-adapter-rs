use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct OutputImageRequest{
	pub outputLocation: String,
}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct OutputImageResponse{
	pub outputFile: String,
	pub format: String,
}


use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct GetAvailableLanguagesRequest {}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct GetAvailableLanguagesResponse{
	pub languages: Vec<String>,
}


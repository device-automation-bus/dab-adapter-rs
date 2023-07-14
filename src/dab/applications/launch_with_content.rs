use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationWithContentRequest {
    pub appId: String,
    pub contentId: String,
    pub parameters: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationWithContentResponse {}

use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationWithContentRequest {
    pub appId: String,
    pub contentId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationWithContentResponse {}

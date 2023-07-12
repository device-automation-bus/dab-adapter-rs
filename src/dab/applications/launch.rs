use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationRequest {
    pub appId: String,
    pub parameters: [String],
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationResponse {}

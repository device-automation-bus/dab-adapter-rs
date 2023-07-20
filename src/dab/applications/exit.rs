use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ExitApplicationRequest {
    pub appId: String,
    pub background: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ExitApplicationResponse {
    pub state: String,
}

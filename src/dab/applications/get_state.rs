use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Deserialize)]
pub struct GetApplicationStateRequest {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct GetApplicationStateResponse {
    pub state: String,
}

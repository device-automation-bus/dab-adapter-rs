use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct GetApplicationStateRequest {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct GetApplicationStateResponse {
    pub state: String,
}

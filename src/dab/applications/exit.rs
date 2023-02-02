use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Deserialize)]
pub struct ExitApplicationRequest {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct ExitApplicationResponse {
    pub state: String,
}

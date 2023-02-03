use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetLanguageRequest {
    pub language: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetLanguageResponse {}

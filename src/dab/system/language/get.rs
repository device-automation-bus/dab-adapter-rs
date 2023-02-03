use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetLanguageRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetLanguageResponse {
    pub language: String,
}

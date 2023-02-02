use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct GetLanguageRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct GetLanguageResponse {
    pub language: String,
}

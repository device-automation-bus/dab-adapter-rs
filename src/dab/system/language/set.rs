use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SetLanguageRequest {
    pub language: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SetLanguageResponse {}

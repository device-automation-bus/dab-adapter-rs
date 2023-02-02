use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SetSystemSettingsRequest {
    pub language: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SetSystemSettingsResponse {
    pub language: String,
}

use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct GetAvailableLanguagesRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct GetAvailableLanguagesResponse {
    pub languages: Vec<String>,
}

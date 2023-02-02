use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct LaunchApplicationWithContentRequest {
    pub appId: String,
    pub contentId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct LaunchApplicationWithContentResponse {}

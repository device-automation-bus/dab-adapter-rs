use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct OutputImageRequest {
    pub outputLocation: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct OutputImageResponse {
    pub outputFile: String,
    pub format: String,
}

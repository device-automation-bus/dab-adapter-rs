use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct LongKeyPressRequest {
    pub keyCode: String,
    pub durationMs: u32,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct KeyPressResponse {}

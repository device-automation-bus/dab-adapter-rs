use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LongKeyPressRequest {
    pub keyCode: String,
    pub durationMs: u32,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct KeyPressResponse {}

use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct KeyPressRequest {
    pub keyCode: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct KeyPressResponse {}

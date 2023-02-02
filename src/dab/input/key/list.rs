use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct KeyListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct KeyList {
    pub keyCodes: Vec<String>,
}

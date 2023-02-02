use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct OperationsListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct ListSupportedOperation {
    pub operations: Vec<String>,
}

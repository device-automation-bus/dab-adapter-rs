// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct OperationsListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ListSupportedOperation{
// pub operations: Vec<String>,
// }

use crate::dab::operations::list::ListSupportedOperation;
#[allow(unused_imports)]
use crate::dab::operations::list::OperationsListRequest;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = ListSupportedOperation::default();
    // *** Fill in the fields of the struct ListSupportedOperation here ***

    ResponseOperator
        .operations
        .push("operations/list".to_string());
    ResponseOperator
        .operations
        .push("applications/list".to_string());
    ResponseOperator
        .operations
        .push("applications/launch".to_string());
    // ResponseOperator
    //     .operations
    //     .push("applications/launch-with-content".to_string());
    ResponseOperator
        .operations
        .push("applications/get-state".to_string());
    ResponseOperator
        .operations
        .push("applications/exit".to_string());
    ResponseOperator.operations.push("device/info".to_string());
    ResponseOperator
        .operations
        .push("system/restart".to_string());
    // ResponseOperator
    //     .operations
    //     .push("system/settings/list".to_string());
    // ResponseOperator
    //     .operations
    //     .push("system/settings/get".to_string());
    // ResponseOperator
    //     .operations
    //     .push("system/settings/set".to_string());
    ResponseOperator
        .operations
        .push("input/key/list".to_string());
    ResponseOperator
        .operations
        .push("input/key-press".to_string());
    // ResponseOperator
    //     .operations
    //     .push("input/long-key-press".to_string());
    ResponseOperator.operations.push("output/image".to_string());
    // ResponseOperator
    //     .operations
    //     .push("device-telemetry/start".to_string());
    // ResponseOperator
    //     .operations
    //     .push("device-telemetry/stop".to_string());
    // ResponseOperator
    //     .operations
    //     .push("app-telemetry/start".to_string());
    // ResponseOperator
    //     .operations
    //     .push("app-telemetry/stop".to_string());
    ResponseOperator
        .operations
        .push("health-check/get".to_string());
    // ResponseOperator.operations.push("voice/list".to_string());
    // ResponseOperator.operations.push("voice/set".to_string());
    // ResponseOperator
    //     .operations
    //     .push("voice/send-audio".to_string());
    // ResponseOperator
    //     .operations
    //     .push("voice/send-text".to_string());
    ResponseOperator.operations.push("version".to_string());
    ResponseOperator
        .operations
        .push("system/language/list".to_string());
    ResponseOperator
        .operations
        .push("system/language/get".to_string());
    ResponseOperator
        .operations
        .push("system/language/set".to_string());
    ResponseOperator.operations.shrink_to_fit();

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

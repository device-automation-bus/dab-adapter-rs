// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ApplicationListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct Application{
// pub appId: String,
// }

use crate::dab::structs::Application;
#[allow(unused_imports)]
use crate::dab::structs::ApplicationListRequest;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = Application::default();
    // *** Fill in the fields of the struct Application here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

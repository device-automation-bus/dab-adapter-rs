// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct VoiceListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct VoiceSystem{
// pub name: String,
// pub enabled: bool,
// }

#[allow(unused_imports)]
use crate::dab::structs::VoiceListRequest;
use crate::dab::structs::VoiceSystem;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = VoiceSystem::default();
    // *** Fill in the fields of the struct VoiceSystem here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct VersionRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct Version{
// pub versions: Vec<String>,
// }

#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::Version;
#[allow(unused_imports)]
use crate::dab::structs::VersionRequest;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = Version::default();
    // *** Fill in the fields of the struct Version here ***

    ResponseOperator.versions.push("2.0".to_string());

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

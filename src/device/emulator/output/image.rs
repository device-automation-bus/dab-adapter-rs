// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct OutputImageRequest{
// pub outputLocation: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct OutputImageResponse{
// pub outputFile: String,
// pub format: String,
// }

#[allow(unused_imports)]
use crate::dab::structs::OutputImageRequest;
use crate::dab::structs::OutputImageResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = OutputImageResponse::default();
    // *** Fill in the fields of the struct OutputImageResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

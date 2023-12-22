use crate::dab::structs::DabError;
use crate::dab::structs::HealthCheckRequest;
use crate::dab::structs::HealthCheckResponse;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: HealthCheckRequest) -> Result < String, DabError > {
    let mut ResponseOperator = HealthCheckResponse::default();
    // *** Fill in the fields of the struct HealthCheckResponse here ***
    ResponseOperator.healthy = true;
    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

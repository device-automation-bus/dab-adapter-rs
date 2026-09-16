use crate::dab::structs::DabError;
use crate::dab::structs::GetPowerModeRequest;
use crate::dab::structs::GetPowerModeResponse;
use crate::device::rdk::system::power_mode::get_rdk_power_mode;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: GetPowerModeRequest) -> Result<String, DabError> {
    let mut ResponseOperator = GetPowerModeResponse::default();
    // *** Fill in the fields of the struct GetPowerModeResponse here ***

    ResponseOperator.powerMode = get_rdk_power_mode()?;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

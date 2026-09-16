use crate::dab::structs::DabError;
use crate::dab::structs::SetPowerModeRequest;
use crate::dab::structs::SetPowerModeResponse;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::RdkResponseSimple;
use crate::device::rdk::system::power_mode::dab_power_mode_to_rdk;
use crate::device::rdk::system::power_mode::get_rdk_power_mode;
use serde::Serialize;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(dab_request: SetPowerModeRequest) -> Result<String, DabError> {
    let mut ResponseOperator = SetPowerModeResponse::default();
    // *** Fill in the fields of the struct SetPowerModeResponse here ***

    //#########org.rdk.System.setPowerState#########
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct SetPowerStateRequestParams {
        powerState: String,
        standbyReason: String,
    }

    let req_params = SetPowerStateRequestParams {
        powerState: dab_power_mode_to_rdk(&dab_request.powerMode).to_string(),
        standbyReason: "DAB_SET_POWER_MODE_REQUEST".to_string(),
    };

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.System.setPowerState", req_params)?;

    // The specification mandates that the current power mode is reported back.
    ResponseOperator.powerMode = get_rdk_power_mode()?;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

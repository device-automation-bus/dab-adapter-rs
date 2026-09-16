pub mod get;
pub mod set;

use crate::dab::structs::DabError;
use crate::dab::structs::PowerMode;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::RdkResponse;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct GetPowerStateResult {
    powerState: String,
}

// Returns the current device power mode, translated from the RDK power state.
pub fn get_rdk_power_mode() -> Result<PowerMode, DabError> {
    //#########org.rdk.System.getPowerState#########
    let rdkresponse: RdkResponse<GetPowerStateResult> =
        rdk_request("org.rdk.System.getPowerState")?;

    match rdkresponse.result.powerState.as_str() {
        "ON" => Ok(PowerMode::Active),
        "STANDBY" | "LIGHT_SLEEP" => Ok(PowerMode::Standby),
        "DEEP_SLEEP" => Ok(PowerMode::DeepSleep),
        state => Err(DabError::Err500(format!(
            "Unknown RDK power state: {}",
            state
        ))),
    }
}

// Translates the DAB power mode to the RDK power state.
pub fn dab_power_mode_to_rdk(power_mode: &PowerMode) -> &'static str {
    match power_mode {
        PowerMode::Active => "ON",
        PowerMode::Standby => "STANDBY",
        PowerMode::DeepSleep => "DEEP_SLEEP",
    }
}

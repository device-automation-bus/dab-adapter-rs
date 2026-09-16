use std::thread;
use tokio::time::Duration;

use crate::dab::structs::DabError;
use crate::dab::structs::FactoryResetRequest;
use crate::dab::structs::FactoryResetResponse;
use crate::device::rdk::interface::get_service_state;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::service_activate;
use crate::device::rdk::interface::RdkResponseSimple;
use serde::Serialize;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: FactoryResetRequest) -> Result<String, DabError> {
    let mut ResponseOperator = FactoryResetResponse::default();
    // *** Fill in the fields of the struct FactoryResetResponse here ***

    //######### Activate org.rdk.Warehouse #########
    if get_service_state("org.rdk.Warehouse")? != "activated" {
        service_activate("org.rdk.Warehouse".to_string())?;
        thread::sleep(Duration::from_millis(500));
    }

    //#########org.rdk.Warehouse.resetDevice#########
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct ResetDeviceRequestParams {
        suppressReboot: bool,
        resetType: String,
    }

    let req_params = ResetDeviceRequestParams {
        suppressReboot: false,
        resetType: "FACTORY".to_string(),
    };

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.Warehouse.resetDevice", req_params)?;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

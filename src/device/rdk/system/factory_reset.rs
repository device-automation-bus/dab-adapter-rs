use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::dab::structs::DabError;
use crate::dab::structs::FactoryResetRequest;
use crate::dab::structs::FactoryResetResponse;
use crate::device::rdk::interface::{get_service_state, is_local_device, service_activate};
use crate::device::rdk::interface::{rdk_request_with_params, RdkResponseSimple};
use serde::Serialize;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: FactoryResetRequest) -> Result<String, DabError> {
    let mut ResponseOperator = FactoryResetResponse::default();
    // *** Fill in the fields of the struct FactoryResetResponse here ***

    if is_local_device() {
        // org.rdk.Warehouse runs deviceReset.sh from inside WPEFramework, and
        // factory-reset.sh stops wpeframework.service, killing itself halfway
        // through. Run it as a transient unit so it lives in its own cgroup.
        let status = Command::new("systemd-run")
            .arg("--no-block")
            .arg("--description=DAB factory reset")
            .arg("sh")
            .arg("/lib/rdk/deviceReset.sh")
            .arg("factory")
            .status()
            .map_err(|e| DabError::Err500(format!("Failed to run systemd-run: {}", e)))?;

        if !status.success() {
            return Err(DabError::Err500(format!(
                "Failed to start the factory reset: systemd-run {}",
                status
            )));
        }

        return Ok(serde_json::to_string(&ResponseOperator).unwrap());
    }

    //######### Activate org.rdk.Warehouse #########
    if get_service_state("org.rdk.Warehouse")? != "activated" {
        service_activate("org.rdk.Warehouse".to_string())?;
        thread::sleep(Duration::from_millis(500));
    }

    //#########org.rdk.Warehouse.resetDevice#########
    #[derive(Serialize)]
    struct ResetDeviceParams {
        suppressReboot: bool,
        resetType: String,
    }

    let req_params = ResetDeviceParams {
        suppressReboot: false,
        resetType: "FACTORY".to_string(),
    };

    // Warehouse runs the reset on a worker thread and answers right away, so
    // the response is published before the device goes down for the reboot.
    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.Warehouse.resetDevice", req_params)?;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

use crate::dab::structs::DabError;
use crate::dab::structs::ExitApplicationRequest;
use crate::dab::structs::ExitApplicationResponse;
use crate::device::rdk::applications::get_state::AppState;
use crate::device::rdk::applications::get_state::get_app_state;
use crate::device::rdk::applications::get_state::get_dab_app_state;
use crate::device::rdk::applications::launch::{rdkshell_suspend, rdkshell_destroy};
use crate::device::rdk::interface::get_lifecycle_timeout;
use std::{thread, time};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: ExitApplicationRequest) -> Result<String, DabError> {
    let mut ResponseOperator = ExitApplicationResponse::default();
    if _dab_request.appId.is_empty() {
        return Err(DabError::Err400(
            "request missing 'appId' parameter".to_string(),
        ));
    }

    // background default is false
    let to_background = _dab_request.background.unwrap_or(false);

    let mut was_stopped = false;
    let app_state = get_app_state(&_dab_request.appId)?;
    match app_state {
        AppState::Visible | AppState::Invisible | AppState::Suspended => {
            if to_background {
                rdkshell_suspend(_dab_request.appId.clone())?;
            } else {
                rdkshell_destroy(_dab_request.appId.clone())?;
            }
        },
        AppState::Hibernated => {
            if to_background == false {
                rdkshell_destroy(_dab_request.appId.clone())?;
            }
        },
        AppState::Stopped => {
            was_stopped = true;
        },
    }

    // *******************************************************************
    for _idx in 1..=8 {
        // 2 seconds (8*250ms)
        // TODO: refactor to listen to Thunder events with websocket.
        thread::sleep(time::Duration::from_millis(250));

        if was_stopped && to_background {
            println!("{} was already STOPPED before putting to BACKGROUND; Exiting loop.", _dab_request.appId);
            wait_till_app_exit_timeout(&_dab_request.appId, "exit_to_background_timeout_ms");
            break;
        }

        ResponseOperator.state = get_dab_app_state(_dab_request.appId.clone())?;

        if is_state_match(&ResponseOperator.state, to_background) {
            let timeout_type = if to_background {
                "exit_to_background_timeout_ms"
            } else {
                "exit_to_destroy_timeout_ms"
            };
            
            wait_till_app_exit_timeout(&_dab_request.appId, timeout_type);
            break;
        }
    }
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

fn wait_till_app_exit_timeout(app_id: &str, timeout_type: &str) {
    let sleep_time = get_lifecycle_timeout(&app_id.to_lowercase(), timeout_type).unwrap_or(2500);
    std::thread::sleep(time::Duration::from_millis(sleep_time));
}

fn is_state_match(state: &str, to_background: bool) -> bool {
    (to_background && (state == "BACKGROUND")) || (!to_background && (state == "STOPPED"))
}

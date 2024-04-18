use crate::dab::structs::DabError;
use crate::dab::structs::LaunchApplicationWithContentRequest;
use crate::device::rdk::applications::launch::move_to_front_set_focus;
use crate::device::rdk::applications::launch::{RDKShellParams,RDKShellRequestParams};
use crate::device::rdk::applications::launch::RdkRequest;
use crate::device::rdk::applications::launch::send_rdkshell_launch_request;
use crate::device::rdk::applications::get_state::get_dab_app_state;
use crate::device::rdk::interface::http_post;
use crate::hw_specific::applications::launch::get_visibility;
use crate::hw_specific::applications::launch::set_visibility;
use crate::hw_specific::applications::launch::wait_till_app_starts;
use serde_json::json;
use urlencoding::decode;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: LaunchApplicationWithContentRequest) -> Result<String, DabError> {
    if _dab_request.appId.is_empty() {
        return Err(DabError::Err400(
            "request missing 'appId' parameter".to_string(),
        ));
    }

    // TODO: expand this to support more apps.
    if !(_dab_request.appId == "Cobalt"
        || _dab_request.appId == "Youtube"
        || _dab_request.appId == "YouTube")
    {
        return Err(DabError::Err400(
            "This operator currently only supports Youtube".to_string(),
        ));
    }

    let launch_req_params = RDKShellRequestParams {
        callsign: _dab_request.appId.clone(),
    };

    let is_cobalt = _dab_request.appId == "Cobalt"
        || _dab_request.appId == "Youtube"
        || _dab_request.appId == "YouTube";

    let mut param_list = vec![];
    // TODO: How to pass contentId to other apps may be different.
    if is_cobalt && !_dab_request.contentId.is_empty() {
        param_list.push(format!("v={}", _dab_request.contentId));
    }

    if let Some(mut parameters) = _dab_request.parameters {
        // TODO: convert received URL encoded list of parameters to matching format to app.
        if is_cobalt {
            // Decode each parameter before appending to the list
            for param in &mut parameters {
                *param = decode(param).unwrap().to_string();
            }
        }
        param_list.append(&mut parameters);
    }

    let mut app_created = true;
    let app_state = get_dab_app_state(_dab_request.appId.clone())?;

    match app_state.as_str() {
        "STOPPED" => {
            // Cold launch of app.
            let req_params = if is_cobalt {
                let url = format!("https://www.youtube.com/tv?{}", param_list.join("&"));
                let config = json!({"url": url});
                RDKShellParams {
                    callsign: _dab_request.appId.clone(),
                    r#type: "Cobalt".into(),
                    configuration: Some(config.to_string()),
                }
            } else {
                // TODO: expand this to support other apps.
                return Err(DabError::Err500(
                    format!("Implementation required to support {} contents.",
                        _dab_request.appId.clone()).to_string(),
                ));
            };
            send_rdkshell_launch_request(req_params)?;
        },
        "BACKGROUND" | "FOREGROUND" => {
            app_created = false;
            // FIXME: If parameters(?) are App startup specific, it may not take effect when resuming "plugin" runtime.
            // Deeplink is required only if you need to pass "content" to the app runtime.
            if param_list.len() > 0 {
                if is_cobalt {
                    // Cobalt plugin specific.
                    let request = RdkRequest {
                        jsonrpc: "2.0".into(),
                        id: 3,
                        method: _dab_request.appId.clone() + ".1.deeplink".into(),
                        params: format!("https://www.youtube.com/tv?{}", param_list.join("&")),
                    };
        
                    let json_string = serde_json::to_string(&request).unwrap();
                    http_post(json_string)?;
                } else {
                    // TODO: Expand to support other App specific deeplinking.
                    return Err(DabError::Err500(
                        format!("Missing {} specific deeplinking implementation.",
                            _dab_request.appId.clone()).to_string(),
                    ));
                }
            }
            // Resume the app runtime.
            let request = RdkRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: "org.rdk.RDKShell.launch".into(),
                params: &launch_req_params,
            };
    
            let json_string = serde_json::to_string(&request).unwrap();
            http_post(json_string)?;
        },
        _ => {
            println!("Should not reach here in any condition. Invalid {:?} App state: {}",
                _dab_request.appId.clone(), app_state.as_str());
        }
    }

    move_to_front_set_focus(_dab_request.appId.clone())?;
    if !get_visibility(_dab_request.appId.clone())? {
        set_visibility(_dab_request.appId.clone(), true)?;
    }

    wait_till_app_starts(_dab_request.appId, app_created)?;

    Ok("{}".to_string())
}
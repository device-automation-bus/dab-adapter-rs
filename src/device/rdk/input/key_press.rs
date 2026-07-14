use crate::dab::structs::DabError;
use crate::dab::structs::KeyPressRequest;
use crate::dab::structs::KeyPressResponse;
use crate::device::rdk::interface::get_keycode;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::input::key::{get_focused_client, needs_direct_injection};
use crate::device::rdk::system::settings::get::{get_rdk_audio_volume, get_rdk_mute};
use crate::device::rdk::system::settings::set::{set_rdk_audio_volume, set_rdk_mute};
use serde::Serialize;

const VOLUME_STEP: u32 = 5;

#[allow(non_snake_case)]
pub fn process(_dab_request: KeyPressRequest) -> Result<String, DabError> {
    let ResponseOperator = KeyPressResponse::default();

    if _dab_request.keyCode.is_empty() {
        return Err(DabError::Err400(
            "request missing 'keyCode' parameter".to_string(),
        ));
    }

    // Handle volume keys via DisplaySettings API to ensure volume actually works
    match _dab_request.keyCode.as_str() {
        "KEY_VOLUME_UP" => {
            let current = get_rdk_audio_volume().unwrap_or(50);
            let new_volume = (current + VOLUME_STEP).min(100);
            let _ = set_rdk_audio_volume(new_volume);
        }
        "KEY_VOLUME_DOWN" => {
            let current = get_rdk_audio_volume().unwrap_or(50);
            let new_volume = current.saturating_sub(VOLUME_STEP);
            let _ = set_rdk_audio_volume(new_volume);
        }
        "KEY_MUTE" => {
            let current_mute = get_rdk_mute().unwrap_or(false);
            let _ = set_rdk_mute(!current_mute);
        }
        _ => {}
    }

    let KeyCode: u16 = match get_keycode(_dab_request.keyCode.clone()) {
        Some(k) => *k,
        None => return Err(DabError::Err400("keyCode not found".to_string())),
    };

    // For volume keys and other intercepted keys, use generateKey with client parameter
    // to bypass platform-level key intercepts. Send to the currently focused app.
    // If no focused app found, skip key injection (volume still changes via DisplaySettings).
    if needs_direct_injection(&_dab_request.keyCode) {
        if let Some(focused_client) = get_focused_client() {
            #[derive(Serialize)]
            struct GenerateKeyRequest {
                jsonrpc: String,
                id: i32,
                method: String,
                params: GenerateKeyParams,
            }

            #[derive(Serialize)]
            struct GenerateKeyParams {
                keys: Vec<KeyEntry>,
            }

            #[derive(Serialize)]
            struct KeyEntry {
                keyCode: u16,
                modifiers: Vec<String>,
                delay: f64,
                client: String,
            }

            let key_entry = KeyEntry {
                keyCode: KeyCode,
                modifiers: vec![],
                delay: 0.0,
                client: focused_client,
            };

            let req_params = GenerateKeyParams {
                keys: vec![key_entry],
            };

            let request = GenerateKeyRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: "org.rdk.RDKShell.1.generateKey".into(),
                params: req_params,
            };

            let json_string = serde_json::to_string(&request).unwrap();
            http_post(json_string)?;
        }

        return Ok(serde_json::to_string(&ResponseOperator).unwrap());
    }

    // Use injectKey for all other keys
    #[derive(Serialize)]
    struct InjectKeyRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: InjectKeyRequestParams,
    }

    #[derive(Serialize)]
    struct InjectKeyRequestParams {
        keyCode: u16,
        modifiers: Vec<String>,
    }

    let req_params = InjectKeyRequestParams {
        keyCode: KeyCode,
        modifiers: vec![],
    };

    let request = InjectKeyRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.1.injectKey".into(),
        params: req_params,
    };

    let json_string = serde_json::to_string(&request).unwrap();
    http_post(json_string)?;

    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

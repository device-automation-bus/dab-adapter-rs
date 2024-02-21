use crate::dab::structs::DabError;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::RdkResponseSimple;
use crate::device::rdk::interface::{ws_close, ws_open, ws_receive, ws_send};
use crate::hw_specific::interface::rdk_request_with_params;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{thread, time};

#[allow(non_snake_case)]
pub fn configureVoice(EnableVoice: bool) -> Result<(), DabError> {
    #[derive(Serialize)]
    struct Ptt {
        enable: bool,
    }

    #[derive(Serialize)]
    struct Param {
        ptt: Ptt,
        enable: bool,
    }

    let req_params = Param {
        enable: EnableVoice,
        ptt: Ptt {
            enable: EnableVoice,
        },
    };

    let rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.VoiceControl.configureVoice", req_params);
    if !rdkresponse.result.success {
        return Err(DabError::Err500(
            "RDK API 'configureVoice' failed.".to_string(),
        ));
    }

    Ok(())
}

fn enable_ptt() -> Result<(), DabError> {
    #[derive(Serialize)]
    struct Ptt {
        enable: bool,
    }

    #[derive(Serialize)]
    struct Param {
        ptt: Ptt,
    }

    let req_params = Param {
        ptt: Ptt { enable: true },
    };

    let rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.VoiceControl.configureVoice", req_params);
    if !rdkresponse.result.success {
        return Err(DabError::Err500(
            "Failed to enable PTT for voice control.".to_string(),
        ));
    }

    Ok(())
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn is_voice_enabled(voiceSystem: String) -> Result<bool, DabError> {
    let mut avs_enabled = false;
    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: String,
    }

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.VoiceControl.voiceStatus".into(),
        params: "{}".into(),
    };

    #[derive(Deserialize)]
    struct RdkResponse {
        jsonrpc: String,
        id: i32,
        result: VoiceStatusResult,
    }

    #[derive(Deserialize)]
    struct VoiceStatusResult {
        capabilities: Vec<String>,
        urlPtt: String,
        urlHf: String,
        prv: bool,
        wwFeedback: bool,
        ptt: SubStatus,
        ff: SubStatus,
        success: bool,
    }

    #[derive(Deserialize)]
    struct SubStatus {
        status: String,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string)?;

    let rdkresponse: RdkResponse = serde_json::from_str(&response_json).unwrap();
    // Current Alexa solution is PTT & starts with protocol 'avs://'
    if rdkresponse.result.urlPtt.to_string().contains("avs:") && voiceSystem == "AmazonAlexa" {
        if rdkresponse.result.ptt.status.to_string().contains("ready") {
            avs_enabled = true;
        }
    }
    Ok(avs_enabled)
}

use tokio::runtime::Runtime;

#[allow(non_snake_case)]
pub fn sendVoiceCommand(audio_file_in: String) -> Result<(), DabError> {
    // Do not configure if already enabled as immediate use may fail.
    let axela_enabled = is_voice_enabled("AmazonAlexa".to_string())?;
    if !axela_enabled {
        match let rdkresponse = enable_ptt() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Register websocket to receive events.
        let mut ws_stream = ws_open().await?;

        let payload = json!({
            "jsonrpc": "2.0",
            "id": "3",
            "method": "org.rdk.VoiceControl.register",
            "params": {
                "event": "onSessionEnd"
            }
        });

        match let wsresponse = ws_send(&mut ws_stream, payload).await {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // Ignore response for now.
        ws_receive(&mut ws_stream).await?;

        #[derive(Serialize)]
        struct Param {
            audio_file: String,
            #[serde(rename = "type")]
            request_type: String,
        }

        let req_params = Param {
            audio_file: audio_file_in,
            request_type: "ptt_audio_file".into(),
        };

        let rdkresponse: RdkResponseSimple =
            rdk_request_with_params("org.rdk.VoiceControl.voiceSessionRequest", req_params);
        if !rdkresponse.result.success {
            return Err(DabError::Err500(
                "RDK API 'voiceSessionRequest' failed.".to_string(),
            ));
        }

        let mut attempts = 0;
        loop {
            let response = ws_receive(&mut ws_stream).await?;

            if response.get("params")
                .and_then(|params| params.get("result"))
                .and_then(|result| result.as_str())
                .map_or(false, |name_str| name_str == "success") {
                ws_close(&mut ws_stream).await?;
                // Tune to match Alexa's breathing and processing time.
                if axela_enabled {
                    println!("Got onSessionEnd.params.result.success; wait for 2sec for Alexa.");
                    thread::sleep(time::Duration::from_secs(2));
                }
                return Ok(());
            }

            attempts += 1;
            if attempts >= 10 {
                ws_close(&mut ws_stream).await?;
                return Err(DabError::Err500(
                    "Timed out waiting for 'onSessionEnd' event.".to_string(),
                ));
            }
        }
    })
}

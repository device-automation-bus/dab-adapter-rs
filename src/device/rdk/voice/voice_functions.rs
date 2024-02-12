use crate::dab::structs::DabError;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::RdkResponseSimple;
use crate::device::rdk::interface::{ws_close, ws_open, ws_receive, ws_send};
use crate::hw_specific::interface::rdk_request_with_params;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.VoiceControl.configureVoice", req_params)?;

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

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.VoiceControl.configureVoice", req_params)?;

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
    let voice_enabled = is_voice_enabled("AmazonAlexa".to_string())?;
    if !voice_enabled {
        enable_ptt()?;
    }

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Register websocket to receive events.
        let mut ws_stream = ws_open().await?;

        // Alexa specific implementation; listen to "onServerMessage" for "RequestProcessingCompleted".
        // {"jsonrpc": "2.0", "method": "client.events.onServerMessage", "params": \
        //   {"xr_speech_avs":{"directive":{"header":{"namespace":"InteractionModel","name":"RequestProcessingCompleted",...},"payload":{}}}}
        // } 
        // For other voice implementation; use "onSessionEnd".
        let payload = json!({
            "jsonrpc": "2.0",
            "id": "3",
            "method": "org.rdk.VoiceControl.register",
            "params": {
                "event": "onServerMessage"
            }
        });

        ws_send(&mut ws_stream, payload).await?;
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

        let _rdkresponse: RdkResponseSimple =
            rdk_request_with_params("org.rdk.VoiceControl.voiceSessionRequest", req_params)?;

        let mut attempts = 0;
        loop {
            let response = ws_receive(&mut ws_stream).await?;

            // Alexa specific response.
            if let Some(params) = response.get("params") {
                if let Some(xr_speech_avs) = params.get("xr_speech_avs") {
                    if let Some(directive) = xr_speech_avs.get("directive") {
                        if let Some(header) = directive.get("header") {
                            if let Some(name) = header.get("name") {
                                if let Some(name_str) = name.as_str() {
                                    if name_str == "RequestProcessingCompleted" {
                                        ws_close(&mut ws_stream).await?;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if attempts >= 5 {
                    ws_close(&mut ws_stream).await?;
                    return Err(DabError::Err500(
                        "Failed to receive onSessionEnd event".to_string(),
                    ));
                }
                attempts += 1;
            }
        }
    })
}

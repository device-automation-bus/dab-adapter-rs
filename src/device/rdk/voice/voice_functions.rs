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
    let alexa_enabled = is_voice_enabled("AmazonAlexa".to_string())?;
    if !alexa_enabled {
        enable_ptt()?;
    }

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Register websocket to receive events.
        let mut ws_stream = ws_open().await?;

        let mut payload = json!({
            "jsonrpc": "2.0",
            "id": "3",
            "method": "org.rdk.VoiceControl.register",
            "params": {
                "event": "onSessionEnd"
            }
        });

        ws_send(&mut ws_stream, payload.clone()).await?;

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
            if cfg!(debug_assertions) {
                println!("Got onSessionEnd: {:?}\n", response.clone());
            }
            // check if response has "method" with "onSessionEnd" and "params" has "result" with "success".
            /* Eg: {"jsonrpc":"2.0","method":"onSessionEnd","params":{
                        "remoteId":255,"result":"success","serverStats":{"connectTime":0,"dnsTime":0,"serverIp":""},
                        "sessionId":"916d763d-ea62-48e9-a527-3a7387ee0352","success":{"transcription":""}
                    }} */
            let response_json: serde_json::Value = serde_json::from_str(&response.to_string()).unwrap();
            if response_json["method"] == "onSessionEnd" && response_json["params"]["result"] == "success" {
                payload["method"] = "org.rdk.VoiceControl.unregister".into();
                ws_send(&mut ws_stream, payload).await?;
                ws_close(&mut ws_stream).await?;
                // Tune to match Alexa's breathing and processing time.
                // ToDo: Replace with a better solution when AVS has proper events.
                if alexa_enabled {
                    println!("Got onSessionEnd.params.result.success; wait for 3sec for Alexa.");
                    thread::sleep(time::Duration::from_secs(3));
                }
                return Ok(());
            }

            attempts += 1;
            if attempts >= 20 {
                payload["method"] = "org.rdk.VoiceControl.unregister".into();
                ws_send(&mut ws_stream, payload).await?;
                ws_close(&mut ws_stream).await?;
                return Err(DabError::Err500(
                    "Timed out waiting for 'onSessionEnd' event.".to_string(),
                ));
            }
        }
    })
}

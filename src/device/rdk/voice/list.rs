#[allow(unused_imports)]
use crate::dab::structs::VoiceListRequest;
use crate::dab::structs::ListVoiceSystemsResponse;

use crate::dab::structs::VoiceSystem;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: VoiceListRequest) -> Result<String, String> {
    let mut ResponseOperator = ListVoiceSystemsResponse::default();
    // *** Fill in the fields of the struct VoiceSystem here ***

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
    let response = http_post(json_string)?;

    let rdkresponse: RdkResponse = serde_json::from_str(&response).unwrap();
    // Current Alexa solution is PTT & starts with protocol 'avs://'
    if rdkresponse.result.urlPtt.to_string().contains("avs:") {
        let mut avsEnabled = false;
        if rdkresponse.result.ptt.status.to_string().contains("ready") {
            avsEnabled = true;
        }
        let avs = VoiceSystem {
            name: ("AmazonAlexa").to_string(),
            enabled: avsEnabled,
        };
        ResponseOperator.voiceSystems.push(avs);
    }

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

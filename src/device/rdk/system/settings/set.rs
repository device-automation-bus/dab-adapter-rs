// pub struct SetSystemSettingsRequest {
//     pub language: String,
//     pub outputResolution: OutputResolution,
//     pub memc: bool,
//     pub cec: bool,
//     pub lowLatencyMode: bool,
//     pub matchContentFrameRate: MatchContentFrameRate,
//     pub hdrOutputMode: HdrOutputMode,
//     pub pictureMode: PictureMode,
//     pub audioOutputMode: AudioOutputMode,
//     pub audioOutputSource: AudioOutputSource,
//     pub videoInputSource: VideoInputSource,
//     pub audioVolume: u32,
//     pub mute: bool,
//     pub textToSpeech: bool,
// }
#[allow(unused_imports)]
use crate::dab::structs::SetSystemSettingsRequest;
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
#[allow(unused_imports)]
use crate::device::rdk::interface::http_post;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use serde_json::Value;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator_json = json!({});
    // *** Fill in the fields of the struct SetSystemSettingsResponse here ***
    let json_str: Value = serde_json::from_str(&_packet).unwrap();

    // ################ outputResolution ################
    // if dab_request.outputResolution != SetSystemSettingsRequest::default().outputResolution {
    if json_str.get("outputResolution").is_some() {
        let dab_request: SetSystemSettingsRequest;
        dab_request = serde_json::from_str(&_packet).unwrap();
        //#########org.rdk.RDKShell.setScreenResolution#########
        #[derive(Serialize)]
        struct Param {
            w: u32,
            h: u32,
        }
        #[derive(Serialize)]
        struct RdkRequest {
            jsonrpc: String,
            id: i32,
            method: String,
            params: Param,
        }

        let req_params = Param {
            w: dab_request.outputResolution.width,
            h: dab_request.outputResolution.height,
        };

        let request = RdkRequest {
            jsonrpc: "2.0".into(),
            id: 3,
            method: "org.rdk.RDKShell.setScreenResolution".into(),
            params: req_params,
        };
        let json_string = serde_json::to_string(&request).unwrap();
        let response_json = http_post(json_string);

        #[derive(Deserialize)]
        struct RdkResponse {
            jsonrpc: String,
            id: i32,
            result: SetScreenResolutionResult,
        }

        #[derive(Deserialize)]
        struct SetScreenResolutionResult {
            success: bool,
        }

        match response_json {
            Err(err) => {
                return Err(err);
            }
            _ => (),
        }
    }

    // ################ audioVolume ################

    // if dab_request.audioVolume != SetSystemSettingsRequest::default().audioVolume {
    if json_str.get("audioVolume").is_some() {
        let dab_request: SetSystemSettingsRequest;
        dab_request = serde_json::from_str(&_packet).unwrap();

        //#########org.rdk.RDKShell.setVolumeLevel#########
        #[derive(Serialize)]
        struct Param {
            volumeLevel: u32,
        }
        #[derive(Serialize)]
        struct RdkRequest {
            jsonrpc: String,
            id: i32,
            method: String,
            params: Param,
        }

        let req_params = Param {
            volumeLevel: dab_request.audioVolume,
        };

        let request = RdkRequest {
            jsonrpc: "2.0".into(),
            id: 3,
            method: "org.rdk.DisplaySettings.setVolumeLevel".into(),
            params: req_params,
        };
        let json_string = serde_json::to_string(&request).unwrap();
        let response_json = http_post(json_string);

        #[derive(Deserialize)]
        struct RdkResponse {
            jsonrpc: String,
            id: i32,
            result: SetVolumeLevelResult,
        }

        #[derive(Deserialize)]
        struct SetVolumeLevelResult {
            success: bool,
        }

        match response_json {
            Err(err) => {
                return Err(err);
            }
            _ => (),
        }
    }

    // *******************************************************************
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

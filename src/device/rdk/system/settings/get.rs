// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct GetSystemSettingsRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct GetSystemSettingsResponse{
// pub language: String,
// pub outputResolution: OutputResolution,
// pub memc: bool,
// pub cec: bool,
// pub lowLatencyMode: bool,
// pub matchContentFrameRate: MatchContentFrameRate,
// pub hdrOutputMode: HdrOutputMode,
// pub pictureMode: PictureMode,
// pub audioOutputMode: AudioOutputMode,
// pub audioOutputSource: AudioOutputSource,
// pub videoInputSource: VideoInputSource,
// pub audioVolume: u32,
// pub mute: bool,
// pub textToSpeech: bool,
// }

#[allow(unused_imports)]
use crate::dab::system::settings::get::GetSystemSettingsRequest;
use crate::dab::system::settings::get::GetSystemSettingsResponse;
#[allow(unused_imports)]
use crate::dab::ErrorResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = GetSystemSettingsResponse::default();
    // *** Fill in the fields of the struct GetSystemSettingsResponse here ***

    //#########org.rdk.RDKShell.getScreenResolution#########
    #[derive(Serialize)]
    struct GetScreenResolutionRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetScreenResolutionRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.getScreenResolution".into(),
    };

    #[derive(Deserialize)]
    struct GetScreenResolutionResponse {
        jsonrpc: String,
        id: i32,
        result: GetScreenResolutionResult,
    }

    #[derive(Deserialize)]
    struct GetScreenResolutionResult {
        w: u32,
        h: u32,
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(response) => {
            let screen_resolution: GetScreenResolutionResponse =
                serde_json::from_str(&response).unwrap();
            ResponseOperator.outputResolution.width = screen_resolution.result.w;
            ResponseOperator.outputResolution.height = screen_resolution.result.h;
        }
    }

    //#########org.rdk.RDKShell.getGraphicsFrameRate#########
    #[derive(Serialize)]
    struct GetGraphicsFrameRateRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetGraphicsFrameRateRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.getGraphicsFrameRate".into(),
    };

    #[derive(Deserialize)]
    struct GetGraphicsFrameRateResponse {
        jsonrpc: String,
        id: i32,
        result: GetGraphicsFrameRateResult,
    }

    #[derive(Deserialize)]
    struct GetGraphicsFrameRateResult {
        frameRate: u32,
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(response) => {
            let get_framerate: GetGraphicsFrameRateResponse =
                serde_json::from_str(&response).unwrap();
            ResponseOperator.outputResolution.frequency = get_framerate.result.frameRate as f32;
        }
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

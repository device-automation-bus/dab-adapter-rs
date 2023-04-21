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
use crate::device::rdk::interface::service_activate;
use crate::device::rdk::interface::service_deactivate;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = GetSystemSettingsResponse::default();
    // *** Fill in the fields of the struct GetSystemSettingsResponse here ***

    //######### outputResolution #########
    service_activate("org.rdk.FrameRate".to_string()).unwrap();
    #[derive(Serialize)]
    struct GetDisplayFrameRateRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetDisplayFrameRateRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.FrameRate.getDisplayFrameRate".into(),
    };

    #[derive(Deserialize)]
    struct GetDisplayFrameRateResponse {
        jsonrpc: String,
        id: i32,
        result: GetDisplayFrameRateResult,
    }

    #[derive(Deserialize)]
    struct GetDisplayFrameRateResult {
        framerate: String,
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
            let get_framerate: GetDisplayFrameRateResponse =
                serde_json::from_str(&response).unwrap();
            let mut dimensions = get_framerate
                .result
                .framerate
                .trim_end_matches(']')
                .split('x');

            ResponseOperator.outputResolution.width =
                dimensions.next().unwrap().parse::<i32>().unwrap() as u32;
            ResponseOperator.outputResolution.height =
                dimensions.next().unwrap().parse::<i32>().unwrap() as u32;
            ResponseOperator.outputResolution.frequency =
                dimensions.next().unwrap().parse::<i32>().unwrap() as f32;
        }
    }
    service_deactivate("org.rdk.RDKShell.getDisplayFrameRate".to_string()).unwrap();

    //######### audioVolume #########
    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.DisplaySettings.getVolumeLevel".into(),
    };
    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    #[derive(Deserialize)]
    struct GetVolumeLevelResponse {
        jsonrpc: String,
        id: i32,
        result: GetVolumeLevelResult,
    }

    #[derive(Deserialize)]
    struct GetVolumeLevelResult {
        volumeLevel: String,
        success: bool,
    }

    match response_json {
        Err(err) => {
            return Err(err);
        }
        Ok(response) => {
            let get_audio_volume: GetVolumeLevelResponse = serde_json::from_str(&response).unwrap();
            let volume = get_audio_volume.result.volumeLevel.parse::<f32>().unwrap();
            ResponseOperator.audioVolume = volume as u32;
        }
    }
    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

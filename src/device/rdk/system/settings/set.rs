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
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::OutputResolution;
#[allow(unused_imports)]
use crate::dab::structs::SetSystemSettingsRequest;
#[allow(unused_imports)]
use crate::device::rdk::system::settings::get::get_rdk_connected_audio_ports;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::RdkResponseSimple;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use serde_json::Value;

fn set_rdk_language(language: String) -> Result<(), String> {
    #[derive(Serialize)]
    struct Param {
        ui_language: String,
    }

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.UserPreferences.1.setUILanguage", Param {
            ui_language: language,
        })?;

    Ok(())
}

fn set_rdk_resolution(resolution: &OutputResolution) -> Result<(), String> {
    #[derive(Serialize)]
    struct Param {
        framerate: String,
    }

    let req_params = Param {
        framerate: format!("{}x{}x{}", resolution.width, resolution.height, resolution.frequency),
    };

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.FrameRate.setDisplayFrameRate", req_params)?;

    Ok(())
}

fn set_rdk_audio_volume (volume: u32) -> Result<(), String> {
    let mut connected_ports = get_rdk_connected_audio_ports()?;

    if connected_ports.is_empty() {
        return Err("Device doesn't have any connected audio port.".into());
    }

    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        volumeLevel: u32,
        audioPort: String,
    }

    let req_params = Param {
        volumeLevel: volume,
        audioPort: connected_ports.remove(0),
    };

    let _rdkresponse: RdkResponseSimple = 
        rdk_request_with_params("org.rdk.DisplaySettings.setVolumeLevel", req_params)?;

    Ok(())
}

#[allow(non_snake_case)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator_json = json!({});
    // *** Fill in the fields of the struct SetSystemSettingsResponse here ***
    let json_str: Value = serde_json::from_str(&_packet).unwrap();

    // ################ outputResolution ################
    // if dab_request.outputResolution != SetSystemSettingsRequest::default().outputResolution {
    if json_str.get("outputResolution").is_some() {
        let dab_request: SetSystemSettingsRequest;
        dab_request = serde_json::from_str(&_packet).unwrap();
        set_rdk_resolution(&dab_request.outputResolution)?;
    }

    // ################ audioVolume ################

    // if dab_request.audioVolume != SetSystemSettingsRequest::default().audioVolume {
    if json_str.get("audioVolume").is_some() {
        let dab_request: SetSystemSettingsRequest;
        dab_request = serde_json::from_str(&_packet).unwrap();
        set_rdk_audio_volume(dab_request.audioVolume)?;
    }

    if json_str.get("language").is_some() {
        let dab_request: SetSystemSettingsRequest;
        dab_request = serde_json::from_str(&_packet).unwrap();

        set_rdk_language(dab_request.language)?;
    }

    // *******************************************************************
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

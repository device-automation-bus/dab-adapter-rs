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
use std::collections::HashMap;
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

fn set_rdk_mute(mute: bool) -> Result<(), String> {
    let mut connected_ports = get_rdk_connected_audio_ports()?;

    if connected_ports.is_empty() {
        return Err("Device doesn't have any connected audio port.".into());
    }

    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        muted: bool,
        audioPort: String,
    }

    let req_params = Param {
        muted: mute,
        audioPort: connected_ports.remove(0),
    };

    let _rdkresponse: RdkResponseSimple = 
        rdk_request_with_params("org.rdk.DisplaySettings.setMuted", req_params)?;

    Ok(())
}

fn set_rdk_cec(enabled: bool) -> Result<(), String> {
    #[derive(Serialize)]
    struct Param {
        enabled: bool,
    }

    let req_params = Param {
        enabled,
    };

    let _rdkresponse: RdkResponseSimple = 
        rdk_request_with_params("org.rdk.HdmiCec_2.setEnabled", req_params)?;

    Ok(())
}

pub fn process(_packet: String) -> Result<String, String> {
    let mut json_map: HashMap<&str, Value> = serde_json::from_str(&_packet).unwrap();

    for (key, value) in json_map.iter_mut() {
        match *key {
            "language" => set_rdk_language(serde_json::from_value::<String>(value.take()).unwrap())?,
            "outputResolution" => set_rdk_resolution(&serde_json::from_value::<OutputResolution>(value.take()).unwrap())?,
            "audioVolume" => set_rdk_audio_volume(serde_json::from_value::<u32>(value.take()).unwrap())?,
            "mute" => set_rdk_mute(value.take().as_bool().unwrap())?,
            "cec" => set_rdk_cec(value.take().as_bool().unwrap())?,
            _ => (),
        }
    };

    Ok(serde_json::to_string(&json!({"status": 200})).unwrap())
}

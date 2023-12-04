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
use crate::dab::structs::AudioOutputMode;
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::OutputResolution;
#[allow(unused_imports)]
use crate::dab::structs::SetSystemSettingsRequest;
#[allow(unused_imports)]
use crate::device::rdk::system::settings::get::get_rdk_audio_port;
use crate::device::rdk::system::settings::list::get_rdk_supported_audio_modes;
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
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        volumeLevel: u32,
        audioPort: String,
    }

    let req_params = Param {
        volumeLevel: volume,
        audioPort: get_rdk_audio_port()?,
    };

    let _rdkresponse: RdkResponseSimple = 
        rdk_request_with_params("org.rdk.DisplaySettings.setVolumeLevel", req_params)?;

    Ok(())
}

fn set_rdk_mute(mute: bool) -> Result<(), String> {
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        muted: bool,
        audioPort: String,
    }

    let req_params = Param {
        muted: mute,
        audioPort: get_rdk_audio_port()?,
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

fn rdk_sound_mode_from_dab(mode: AudioOutputMode, port: &String) -> Result<String, String> {
    use AudioOutputMode::*;

    match mode {
        Stereo => Ok("STEREO".into()),
        PassThrough => Ok("PASSTHRU".into()),
        Auto => Ok("AUTO".into()),
        MultichannelPcm => {
            get_rdk_supported_audio_modes(port)?
                .iter()
                .find(|mode| {
                    ["SURROUND", "DOLBYDIGITAL", "DOLBYDIGITALPLUS"].contains(&mode.as_str())
                })
                .cloned()
                .ok_or("Audio port doesn't support multichannel.".into())
        }
    }
}

fn set_rdk_audio_output_mode(mode: AudioOutputMode) -> Result<(), String> {
    #[allow(non_snake_case)]
    #[derive(Default, Serialize)]
    struct Param {
        audioPort: String,
        soundMode: String,
    }

    let mut req_params = Param {
        audioPort: get_rdk_audio_port()?,
        ..Default::default()
    };
    req_params.soundMode = rdk_sound_mode_from_dab(mode, &req_params.audioPort)?;

    let _rdkresponse: RdkResponseSimple = 
        rdk_request_with_params("org.rdk.DisplaySettings.setSoundMode", req_params)?;

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
            "audioOutputMode" => set_rdk_audio_output_mode(serde_json::from_value::<AudioOutputMode>(value.take()).unwrap())?,
            "audioOutputSource" | "videoInputSource" | _ => return Err(format!("Setting '{}' is not supported", key)),
        }
    };

    Ok(serde_json::to_string(&json!({"status": 200})).unwrap())
}

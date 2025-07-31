use crate::dab::structs::AudioOutputMode;
use crate::dab::structs::AudioOutputSource;
use crate::dab::structs::DabError;
use crate::dab::structs::HdrOutputMode;
use crate::dab::structs::OutputResolution;
use crate::dab::structs::SetSystemSettingsRequest;
use crate::device::rdk::interface::rdk::rdk_request_with_params;
use crate::device::rdk::interface::rdk::RdkResponseSimple;

use crate::device::rdk::system::settings::get::get_rdk_audio_port;
use crate::device::rdk::system::settings::get::get_rdk_hdr_current_setting;
use crate::device::rdk::system::settings::list::get_rdk_hdr_settings;
use crate::device::rdk::system::settings::list::get_rdk_supported_audio_modes;
use crate::hw_specific::configuration::get_audio_volume_range;
use crate::hw_specific::system::settings::get::get_rdk_connected_video_displays;

use serde::{Deserialize, Serialize};

use serde_json::Value;
use std::collections::HashMap;

fn set_rdk_language(language: String) -> Result<(), DabError> {
    #[derive(Serialize)]
    struct Param {
        ui_language: String,
    }

    let _rdkresponse: RdkResponseSimple = rdk_request_with_params(
        "org.rdk.UserPreferences.1.setUILanguage",
        Param {
            ui_language: language,
        },
    )?;

    Ok(())
}

pub fn convert_resolution_to_string(resolution: &OutputResolution) -> Result<String, DabError> {
    let resolution_map = [
        ([640, 480], "480"),
        ([720, 576], "576"),
        ([1280, 720], "720"),
        ([1920, 1080], "1080"),
        ([3840, 2160], "2160"),
    ];
    for (res_arr, res_str) in &resolution_map {
        if resolution.width == res_arr[0] && resolution.height == res_arr[1] {
            return Ok(format!("{}p{}", res_str, resolution.frequency));
        }
    }
    Err(DabError::Err500("Unsupported video format".to_string()))
}

fn set_rdk_resolution(resolution: &OutputResolution) -> Result<(), DabError> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Serialize, Deserialize)]
    struct Param {
        videoDisplay: String,
        resolution: String,
        persist: bool,
        // ignoreEdid: bool, Optional parameter; expected to fail the set call if resolution is not supported by the sink.
    }

    let req_params = Param {
        videoDisplay: get_rdk_connected_video_displays()?,
        resolution: convert_resolution_to_string(resolution)?,
        persist: true,
        // ignoreEdid: false, Optional parameter; expected to fail the set call if resolution is not supported by the sink.
    };

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.DisplaySettings.setCurrentResolution", req_params)?;

    Ok(())
}

fn set_rdk_audio_volume(volume: u32) -> Result<(), DabError> {

    let range = get_audio_volume_range();
    if !(range.min..=range.max).contains(&volume) {
        return Err(DabError::Err400("Unsupported audio volume value".to_string()))
    }

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

fn set_rdk_mute(mute: bool) -> Result<(), DabError> {
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

fn set_rdk_cec(enabled: bool) -> Result<(), DabError> {
    #[derive(Serialize)]
    struct Param {
        enabled: bool,
    }

    let req_params = Param { enabled };

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.HdmiCecSource.setEnabled", req_params)?;

    Ok(())
}

fn set_rdk_audio_output_source(source: AudioOutputSource) -> Result<(), DabError> {
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        audioPort: String,
        enable: bool,
    }

    let mut req_params = Param {
        audioPort: serde_json::to_string(&source).unwrap(),
        enable: true,
    };

    if source == AudioOutputSource::HDMI {
        req_params.audioPort = "HDMI0".to_string();
    }

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.DisplaySettings.setEnableAudioPort", req_params)?;
    Ok(())
}

fn set_rdk_hdr_mode(mode: HdrOutputMode) -> Result<(), DabError> {
    match get_rdk_hdr_settings() {
        Ok(supported_modes) => {
            if !supported_modes.contains(&mode) {
                return Err(DabError::Err400(format!(
                    "Setting hdr mode '{:?}' is not supported", mode)));
            }
        },
        Err(err) => {
            return Err(err);
        }
    }

    match get_rdk_hdr_current_setting() {
        Ok(current_mode) => {
            if mode == current_mode {
                return Ok(());
            }
        },
        Err(err) => {
            return Err(err);
        }
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Default)]
    struct Param {
        hdr_mode: bool,
    }

    let mut req_params = Param::default();

    match mode {
        // STB HDR mode is always enable
        HdrOutputMode::AlwaysHdr => {
            req_params.hdr_mode = true;
        }
        HdrOutputMode::HdrOnPlayback => {
            return Err(DabError::Err500(format!(
                "Setting hdr mode '{}' is not supported",
                "HdrOnPlayback"
            )))
        }
        HdrOutputMode::DisableHdr => {
            req_params.hdr_mode = false;
        }
    }

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.DisplaySettings.setForceHDRMode", req_params)?;
    Ok(())
}

fn rdk_sound_mode_from_dab(mode: AudioOutputMode, port: &String) -> Result<String, DabError> {
    use AudioOutputMode::*;

    match mode {
        Stereo => Ok("STEREO".to_string()),
        PassThrough => Ok("PASSTHRU".to_string()),
        Auto => Ok("AUTO".to_string()),
        MultichannelPcm => get_rdk_supported_audio_modes(port)?
            .iter()
            .find(|mode| ["SURROUND", "DOLBYDIGITAL", "DOLBYDIGITALPLUS"].contains(&mode.as_str()))
            .cloned()
            .ok_or(DabError::Err500(
                "Audio port doesn't support multichannel.".to_string(),
            )),
    }
}

fn set_rdk_audio_output_mode(mode: AudioOutputMode) -> Result<(), DabError> {
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

fn set_rdk_text_to_speech(val: bool) -> Result<(), DabError> {
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        enabletts: bool,
    }

    let req_params = Param { enabletts: val };

    let _rdkresponse: RdkResponseSimple =
        rdk_request_with_params("org.rdk.TextToSpeech.enabletts", req_params)?;

    Ok(())
}

pub fn process(_dab_request: SetSystemSettingsRequest) -> Result<String, DabError> {
    let _packet = serde_json::to_string(&_dab_request).unwrap();
    let mut json_map: HashMap<&str, Value> = serde_json::from_str(&_packet).unwrap();

    for (key, value) in json_map.iter_mut() {
        match *key {
            "language" => {
                set_rdk_language(serde_json::from_value::<String>(value.take()).unwrap())?
            }
            "outputResolution" => set_rdk_resolution(
                &serde_json::from_value::<OutputResolution>(value.take()).unwrap(),
            )?,
            "audioVolume" => {
                set_rdk_audio_volume(serde_json::from_value::<u32>(value.take()).unwrap())?
            }
            "mute" => set_rdk_mute(value.take().as_bool().unwrap())?,
            "cec" => set_rdk_cec(value.take().as_bool().unwrap())?,
            "audioOutputMode" => set_rdk_audio_output_mode(
                serde_json::from_value::<AudioOutputMode>(value.take()).unwrap(),
            )?,
            "audioOutputSource" => set_rdk_audio_output_source(
                serde_json::from_value::<AudioOutputSource>(value.take()).unwrap(),
            )?,
            "hdrOutputMode" => {
                set_rdk_hdr_mode(serde_json::from_value::<HdrOutputMode>(value.take()).unwrap())?
            }
            "textToSpeech" => set_rdk_text_to_speech(value.take().as_bool().unwrap())?,
            "pictureMode" | "videoInputSource" | "lowLatencyMode" | _ => {
                return Err(DabError::Err400(format!(
                    "Setting '{}' is not supported",
                    key
                )))
            }
        }
    }

    Ok("{}".to_string())
}

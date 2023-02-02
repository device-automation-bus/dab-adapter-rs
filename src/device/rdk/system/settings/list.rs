// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SettingsListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct OutputResolution{
	// pub width: u32,
	// pub height: u32,
	// pub frequency: f32,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum MatchContentFrameRate{#[default]
	//     EnabledAlways,
	//     EnabledSeamlessOnly,
	//     Disabled,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum HdrOutputMode{#[default]
	//     AlwaysHdr,
	//     HdrOnPlayback,
	//     DisableHdr,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum PictureMode{#[default]
	//     Standard,
	//     Dynamic,
	//     Movie,
	//     Sports,
	//     FilmMaker,
	//     Game,
	//     Auto,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum AudioOutputMode{#[default]
	//     Stereo,
	//     MultichannelPcm,
	//     PassThrough,
	//     Auto,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum AudioOutputSource{#[default]
	//     NativeSpeaker,
	//     Arc,
	//     EArc,
	//     Optical,
	//     Aux,
	//     Bluetooth,
	//     Auto,
	//     HDMI,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum VideoInputSource{#[default]
	//     Tuner,
	//     HDMI1,
	//     HDMI2,
	//     HDMI3,
	//     HDMI4,
	//     Composite,
	//     Component,
	//     Home,
	//     Cast,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct AudioVolume{
	// pub min: u32,
	// pub max: u32,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct ListSystemSettings{
	// pub language: Vec<String>,
	// pub outputResolution: Vec<OutputResolution>,
	// pub memc: bool,
	// pub cec: bool,
	// pub lowLatencyMode: bool,
	// pub matchContentFrameRate: Vec<MatchContentFrameRate>,
	// pub hdrOutputMode: Vec<HdrOutputMode>,
	// pub pictureMode: Vec<PictureMode>,
	// pub audioOutputMode: Vec<AudioOutputMode>,
	// pub audioOutputSource: Vec<AudioOutputSource>,
	// pub videoInputSource: Vec<VideoInputSource>,
	// pub audioVolume: AudioVolume,
	// pub mute: bool,
	// pub textToSpeech: bool,
// }

use serde::{Deserialize, Serialize};
use crate::device::rdk::interface::http_post;
use serde_json::json;
# [allow(unused_imports)]
use crate::dab::ErrorResponse;
# [allow(unused_imports)]
use crate::dab::system::settings::list::SettingsListRequest;
#[allow(unused_imports)]
use crate::dab::system::settings::list::OutputResolution;
#[allow(unused_imports)]
use crate::dab::system::settings::list::AudioVolume;
use crate::dab::system::settings::list::ListSystemSettings;

# [allow(non_snake_case)]
# [allow(dead_code)]
# [allow(unused_mut)]
pub fn process (_packet: String) -> Result<String,String> {
	let mut ResponseOperator = ListSystemSettings::default();
	// *** Fill in the fields of the struct ListSystemSettings here ***
	
	#[derive(Serialize)]
	struct RdkRequest {
		jsonrpc: String,
		id: i32,
		method: String,
		params: String,
	}

	
	let request = RdkRequest {
		        jsonrpc : "2.0".into(),
		        id      : 3,
		        method  : "org.rdk.DisplaySettings.getConnectedVideoDisplays".into(),
		        params  : "{}".into(),

	    };

	
	#[derive(Deserialize)]
	struct RdkResponse {
		jsonrpc: String,
		id: i32,
		result: GetConnectedVideoDisplaysResult,
	}

	
	#[derive(Deserialize)]
	struct GetConnectedVideoDisplaysResult {
		connectedVideoDisplays: Vec<String>,
		success: bool,
	}

	
	    let json_string = serde_json::to_string(&request).unwrap();
	    let response_json = http_post(json_string);
	
	    match response_json {
		        Ok(val2) => {
			let rdkresponse: RdkResponse = serde_json::from_str(&val2).unwrap();
			            println!("Sucesso: {}", rdkresponse.result.success);

		        }

		        Err(err) => {
			            println!("Erro: {}", err);

			return Err(err)
		        }

	    }

	
	// *******************************************************************
	let mut ResponseOperator_json = json!(ResponseOperator);
	ResponseOperator_json["status"] = json!(200);
	Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}


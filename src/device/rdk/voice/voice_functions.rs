#[allow(unused_imports)]
use crate::dab::structs::SendTextRequest;
use crate::dab::structs::VoiceTextRequestResponse;
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn encode_adpcm(samples: &[i16]) -> Vec<u8> {
    let adpcm_step_table: [i16; 89] = [
        7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60,
        66, 73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371,
        408, 449, 494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878,
        2066, 2272, 2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845,
        8630, 9493, 10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086,
        29794, 32767,
    ];

    let adpcm_index_table: [i16; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];

    let mut adpcm_data = Vec::new();
    let mut frame_seq_num = 0u8;
    let mut sample = 0i16;
    let mut index = 0i16;
    let mut nibble = true;
    let mut byte = 0u8;
    let mut frame_count = 0;

    for &next_sample in samples {
        let mut diff = next_sample - sample;
        let mut step = adpcm_step_table[index as usize];
        let mut vpdiff = step >> 3;
        let mut code = 0;

        if diff < 0 {
            code |= 8;
            diff = -diff;
        }

        if diff >= step {
            code |= 4;
            diff -= step;
            vpdiff += step;
        }

        step >>= 1;

        if diff >= step {
            code |= 2;
            diff -= step;
            vpdiff += step;
        }

        step >>= 1;

        if diff >= step {
            code |= 1;
            vpdiff += step;
        }

        if (code & 8) != 0 {
            sample -= vpdiff;
        } else {
            sample += vpdiff;
        }

        index += adpcm_index_table[code as usize];
        if index < 0 {
            index = 0;
        } else if index > 88 {
            index = 88;
        }

        if nibble {
            byte = (code << 4) & 0xF0;
            nibble = false;
        } else {
            byte |= code & 0x0F;
            adpcm_data.push(byte);
            nibble = true;
            frame_count += 1;
        }

        if frame_count == 96 {
            let metadata = vec![
                frame_seq_num,
                index as u8,
                (sample & 0xFF) as u8,
                ((sample >> 8) & 0xFF) as u8,
            ];

            frame_seq_num = frame_seq_num.wrapping_add(1); // Increment frame sequence number and wrap around at 256
            adpcm_data.splice(
                adpcm_data.len() - 96..adpcm_data.len() - 96,
                metadata.into_iter(),
            );
            frame_count = 0;
        }
    }

    // Add the last byte if there is an odd number of samples
    if !nibble {
        adpcm_data.push(byte);
    }

    // Add metadata for the last frame if needed
    if frame_count > 0 {
        let metadata = vec![
            frame_seq_num,
            index as u8,
            (sample & 0xFF) as u8,
            ((sample >> 8) & 0xFF) as u8,
        ];

        adpcm_data.splice(
            adpcm_data.len() - frame_count..adpcm_data.len() - frame_count,
            metadata.into_iter(),
        );
    }

    adpcm_data
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn sendVoiceCommand() -> Result<String, String> {
    let mut ResponseOperator = VoiceTextRequestResponse::default();
    // *** Fill in the fields of the struct VoiceTextRequestResponse here ***
    extern crate hound;
    use std::fs::File;
    use std::io::Write;

    let reader = hound::WavReader::open("/tmp/tts.wav").unwrap();
    let samples = reader.into_samples::<i16>().map(|x| x.unwrap());
    let samples: Vec<i16> = samples.collect();

    let buffer = encode_adpcm(&samples);
    let mut file = File::create("/tmp/tts.raw").unwrap();
    file.write_all(&buffer).unwrap();

    // **************************** getConnectedDevices ****************************************
    #[derive(Serialize)]
    struct GetConnectedDevicesRequest {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = GetConnectedDevicesRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.Bluetooth.getConnectedDevices".into(),
    };

    #[derive(Deserialize)]
    struct GetConnectedDevicesResponse {
        jsonrpc: String,
        id: i32,
        result: GetConnectedDevicesResult,
    }

    #[derive(Deserialize)]
    struct GetConnectedDevicesResult {
        connectedDevices: Vec<ConnectedDevices>,
        success: bool,
    }

    #[derive(Deserialize)]
    struct ConnectedDevices {
        deviceID: String,
        name: String,
        deviceType: String,
        activeState: String,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    let mut _thisDeviceID = String::new();
    match response_json {
        Ok(val1) => {
            let rdkresponse: GetConnectedDevicesResponse = serde_json::from_str(&val1).unwrap();

            for connectedDevice in rdkresponse.result.connectedDevices.iter() {
                if connectedDevice.deviceType == "HUMAN INTERFACE DEVICE" {
                    _thisDeviceID = connectedDevice.deviceID.clone();
                    break;
                }
            }
            if _thisDeviceID.len() == 0 {
                let err = "No bluetooth remote control found.".to_string();
                return Err(err);
            }
        }
        Err(err) => {
            println!("Erro: {}", err);
            return Err(err);
        }
    }

    // ****************************** getDeviceInfo **************************************

    #[derive(Serialize)]
    struct GetDeviceInfoRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: GetDeviceInfoParams,
    }

    #[derive(Serialize)]
    struct GetDeviceInfoParams {
        deviceID: String,
    }

    let request = GetDeviceInfoRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.Bluetooth.getDeviceInfo".into(),
        params: {
            GetDeviceInfoParams {
                deviceID: _thisDeviceID,
            }
        },
    };

    #[derive(Deserialize)]
    struct GetDeviceInfoResponse {
        jsonrpc: String,
        id: i32,
        result: GetDeviceInfoResult,
    }

    #[derive(Deserialize)]
    struct GetDeviceInfoResult {
        deviceInfo: DeviceInfo,
        success: bool,
    }
    #[derive(Deserialize)]
    struct DeviceInfo {
        deviceID: String,
        name: String,
        deviceType: String,
        supportedProfile: String,
        manufacturer: String,
        MAC: String,
        rssi: String,
        signalStrength: String,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    let mut _thisDeviceMAC = String::new();

    match response_json {
        Ok(val2) => {
            let rdkresponse: GetDeviceInfoResponse = serde_json::from_str(&val2).unwrap();
            let thisDeviceMACResponse = rdkresponse.result.deviceInfo.MAC.clone().replace(":", "");
            _thisDeviceMAC = String::from("0x");
            _thisDeviceMAC.insert_str(2, &thisDeviceMACResponse);
        }

        Err(err) => {
            println!("Erro: {}", err);

            return Err(err);
        }
    }

    // ****************************** voiceSessionBegin **************************************

    #[derive(Serialize)]
    struct VoiceSessionBeginRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: VoiceSessionBeginParams,
    }

    #[derive(Serialize)]
    struct VoiceSessionBeginParams {
        MacAddr: String,
        AudioFile: String,
    }

    let request = VoiceSessionBeginRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.VoiceControl.1.voiceSessionBegin".into(),
        params: {
            VoiceSessionBeginParams {
                MacAddr: _thisDeviceMAC,
                AudioFile: "/tmp/tts.raw".into(),
            }
        },
    };

    #[derive(Deserialize)]
    struct RdkResponse {
        jsonrpc: String,
        id: i32,
        result: RdkResult,
    }

    #[derive(Deserialize)]
    struct RdkResult {
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    match response_json {
        Ok(_) => {}

        Err(err) => {
            println!("Erro: {}", err);

            return Err(err);
        }
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

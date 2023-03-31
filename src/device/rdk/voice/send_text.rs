// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SendTextRequest{
// pub requestText: String,
// pub voiceSystem: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct VoiceTextRequestResponse {}

#[allow(unused_imports)]
use crate::dab::voice::send_text::SendTextRequest;
use crate::dab::voice::send_text::VoiceTextRequestResponse;
#[allow(unused_imports)]
use crate::dab::ErrorResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;


fn encode_adpcm(samples: &[i16]) -> Vec<u8> {

    let adpcm_step_table: [i16; 89] = [ 7, 8, 9, 10, 11, 12, 13, 14, 16, 17,
                            19, 21, 23, 25, 28, 31, 34, 37, 41, 45,
                            50, 55, 60, 66, 73, 80, 88, 97, 107, 118,
                            130, 143, 157, 173, 190, 209, 230, 253, 279, 307,
                            337, 371, 408, 449, 494, 544, 598, 658, 724, 796,
                            876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066,
                            2272, 2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358,
                            5894, 6484, 7132, 7845, 8630, 9493, 10442, 11487, 12635, 13899,
                            15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767];

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
            adpcm_data.splice(adpcm_data.len() - 96..adpcm_data.len() - 96, metadata.into_iter());
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

        adpcm_data.splice(adpcm_data.len() - frame_count..adpcm_data.len() - frame_count, metadata.into_iter());
    }

    adpcm_data
}


#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(packet: String) -> Result<String, String> {
    let mut ResponseOperator = VoiceTextRequestResponse::default();
    // *** Fill in the fields of the struct VoiceTextRequestResponse here ***
    use tts_rust::{ tts::GTTSClient, languages::Languages };
    use std::process::Command;

    let IncomingMessage = serde_json::from_str(&packet);

    match IncomingMessage {
        Err(err) => {
            let response = ErrorResponse {
                status: 400,
                error: "Error parsing request: ".to_string() + err.to_string().as_str(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
        _ => (),
    }

    let Dab_Request: SendTextRequest = IncomingMessage.unwrap();

    if Dab_Request.requestText.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'requestText' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    let narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English,
        tld: "com",
    };
    narrator.save_to_file(&Dab_Request.requestText, "/tmp/tts.mp3")
        .expect("Failed to save to file");

    let mut child = Command::new("gst-launch-1.0")
    .arg("-q")
    .arg("filesrc")
    .arg("location=/tmp/tts.mp3")
    .arg("!")
    .arg("decodebin")
    .arg("!")
    .arg("audioconvert")
    .arg("!")
    .arg("audioresample")
    .arg("!")
    .arg("audio/x-raw,rate=16000,channels=1,format=S16LE")
    .arg("!")
    .arg("wavenc")
    .arg("!")
    .arg("filesink")
    .arg("location=/tmp/tts.wav")
    .spawn()
    .expect("Failed to execute command");
    
    child.wait().expect("failed to wait for child process");


    extern crate hound;
    use std::fs::File;
    use std::io::Write;

    let reader = hound::WavReader::open("/tmp/tts.wav").unwrap();
    let samples = reader.into_samples::<i16>().map(|x| x.unwrap());
    let samples: Vec<i16> = samples.collect();

    let buffer = encode_adpcm(&samples);
    let mut file = File::create("/tmp/tts.raw").unwrap();
    file.write_all(&buffer).unwrap();

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
        params: {VoiceSessionBeginParams{
            MacAddr: "0x0".into(),
            AudioFile: "/tmp/tts.raw".into()
        }},
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
        Ok(val2) => {
            let _rdkresponse: RdkResponse = serde_json::from_str(&val2).unwrap();
        }

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

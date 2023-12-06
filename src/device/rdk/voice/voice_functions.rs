#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
#[allow(unused_imports)]
use crate::dab::structs::SendTextRequest;
use crate::device::rdk::interface::RdkResponseSimple;
use crate::hw_specific::interface::rdk_request_with_params;
use serde::Serialize;

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

// fn enable_voice(EnableVoice: bool) -> Result<(), RdkResponseSimple> {
//    #[derive(Serialize)]
//    struct Ptt {
//        enable: bool,
//    }
//
//    #[derive(Serialize)]
//    struct Param {
//        ptt: Ptt,
//        enable: bool,
//    }
//
//    let req_params = Param {
//        enable: EnableVoice,
//        ptt: Ptt { enable: true },
//    };
//
//    let _rdkresponse: RdkResponseSimple = rdk_request_with_params ("org.rdk.VoiceControl.configureVoice", req_params)?;
//
//    Ok(())
//}

fn enable_ptt() -> Result<(), String> {
    #[derive(Serialize)]
    struct Ptt {
        enable: bool,
    }

    #[derive(Serialize)]
    struct Param {
        ptt: Ptt,
    }

    let req_params = Param {
        ptt: Ptt { enable: true },
    };

    let _rdkresponse: RdkResponseSimple = rdk_request_with_params ("org.rdk.VoiceControl.configureVoice", req_params)?;

    Ok(())
}

#[allow(non_snake_case)]
pub fn sendVoiceCommand() -> Result<(), String> {
    enable_ptt()?;

    #[derive(Serialize)]
    struct Param {
        audio_file: String,
        #[serde(rename = "type")]
        request_type: String,
    }

    let req_params = Param {
        audio_file: "/tmp/tts.wav".into(),
        request_type: "ptt_audio_file".into(),
    };

    let _rdkresponse: RdkResponseSimple = rdk_request_with_params ("org.rdk.VoiceControl.voiceSessionRequest", req_params)?;

    Ok(())
}

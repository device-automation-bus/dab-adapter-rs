// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct OutputImageRequest{
// pub outputLocation: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct OutputImageResponse{
// pub outputFile: String,
// pub format: String,
// }

#[allow(unused_imports)]
use crate::dab::structs::OutputImageRequest;
use crate::dab::structs::OutputImageResponse;
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::service_activate;
use serde::{Deserialize, Serialize};
use serde_json::json;

// use hyper::{Body, Request, Response};
use local_ip_address::local_ip;
// use std::convert::Infallible;
// use std::fs::File;
// use std::io::prelude::*;
use std::thread;
use std::time::Duration;
// use tiff::encoder::{colortype, compression::*, TiffEncoder};

use image::{GenericImageView, ImageResult};

fn is_png_empty(file_path: &str) -> ImageResult<bool> {
    let img = image::open(file_path)?;
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            if pixel.0[3] != 0 {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = OutputImageResponse::default();
    // *** Fill in the fields of the struct OutputImageResponse here ***

    let my_local_ip = local_ip().unwrap();
    let my_server: String = "http://".to_string() + &my_local_ip.to_string() + &":7878".to_string();

    let IncomingMessage = serde_json::from_str(&_packet);

    match IncomingMessage {
        Err(err) => {
            let response = ErrorResponse {
                status: 400,
                error: "Error parsing request: ".to_string() + err.to_string().as_str(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
        Ok(_) => (),
    }

    let Dab_Request: OutputImageRequest = IncomingMessage.unwrap();

    if Dab_Request.outputLocation.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'outputLocation' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    service_activate("org.rdk.ScreenCapture".to_string()).unwrap();

    //#########org.rdk.ScreenCapture.uploadScreenCapture#########
    #[derive(Serialize)]
    struct UploadScreenCaptureRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: UploadScreenCaptureRequestParams,
    }

    #[derive(Serialize)]
    struct UploadScreenCaptureRequestParams {
        url: String,
        callGUID: String,
    }

    let req_params = UploadScreenCaptureRequestParams {
        url: my_server,
        callGUID: "12345".to_string(),
    };

    let request = UploadScreenCaptureRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.ScreenCapture.uploadScreenCapture".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct UploadScreenCaptureResponse {
        jsonrpc: String,
        id: i32,
        result: UploadScreenCaptureResult,
    }

    #[derive(Deserialize)]
    struct UploadScreenCaptureResult {
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string.clone());

    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(_) => {}
    }

    thread::sleep(Duration::from_secs(10));

    let file_path = "/tmp/screenshot.png";
    let is_empty = is_png_empty(file_path).unwrap();
    if is_empty {
        let error = ErrorResponse {
            status: 500,
            error: "Screenshot is empty. Maybe DRM protected?".to_string(),
        };
        return Err(serde_json::to_string(&error).unwrap());
    }

    // let result_upload = upload_image(Dab_Request.outputLocation.clone());

    // match result_upload {
    //     Err(err) => {
    //         let error = ErrorResponse {
    //             status: 500,
    //             error: err,
    //         };
    //         return Err(serde_json::to_string(&error).unwrap());
    //     }
    //     Ok(_) => {}
    // }

    //######### Correlate Fields #########
    ResponseOperator.format = "tiff".to_string();
    ResponseOperator.outputFile = Dab_Request.outputLocation;

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

// pub async fn save_image(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     // Get the body of the request and save the body to a file
//     let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
//     let mut file = File::create("/tmp/screenshot.png").unwrap();
//     file.write_all(&body).unwrap();

//     // Open the image
//     let input_png = image::open("/tmp/screenshot.png").unwrap();
//     let rgb_image = input_png.clone().into_rgba8();
//     let buffer = rgb_image.clone().into_raw();

//     // Decode to tiff
//     let mut file = File::create("/tmp/screenshot.tiff").unwrap();
//     let mut tiff_encodder = TiffEncoder::new(&mut file).unwrap();
//     let mut output_image = tiff_encodder
//         .new_image_with_compression::<colortype::RGBA8, Uncompressed>(
//             1920,
//             1080,
//             Uncompressed::default(),
//         )
//         .unwrap();

//     let mut idx = 0;
//     while output_image.next_strip_sample_count() > 0 {
//         let sample_count = output_image.next_strip_sample_count() as usize;
//         output_image
//             .write_strip(&buffer[idx..idx + sample_count])
//             .unwrap();
//         idx += sample_count;
//     }
//     output_image.finish().unwrap();

//     Ok(Response::new(Body::empty()))
// }

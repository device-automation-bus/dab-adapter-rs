use std::{fs::File, io::Write};

use surf::Client;
use crate::{dab::structs::DabError, hw_specific::interface::{get_ip_address, get_is_debug}};
use futures::executor::block_on;

pub fn http_download(url: String) -> Result<(), DabError> {
    let client = Client::new();

    let response = block_on(async { client.get(url).await });

    match response {
        Ok(mut r) => {
            let mut file = File::create("/tmp/tts.wav").unwrap();
            let body = block_on(r.body_bytes()).unwrap();
            file.write_all(&body).unwrap();
            return Ok(());
        }
        Err(err) => return Err(DabError::Err500(err.to_string())),
    }
}

pub fn http_post(json_string: String) -> Result<String, DabError> {
    let client = Client::new();
    let rdk_address = format!("http://{}:9998/jsonrpc", get_ip_address());

    if get_is_debug() {
        println!("RDK request: {}", json_string);
    }

    let response = block_on(async {
        match client
            .post(rdk_address)
            .body_string(json_string)
            .header("Content-Type", "application/json")
            .await
        {
            Ok(mut response) => {
                match response.body_string().await {
                    Ok(body) => Ok(body),
                    Err(e) => Err(format!("Error while getting the body: {}",e)),
                }
            }
            Err(e) => Err(format!("Error while sending the request: {}",e)),
        }
    });

    match response {
        Ok(r) => {
            let str = r.to_string();

            if get_is_debug() {
                println!("RDK response: {}", str);
            }

            return Ok(str);
        }
        Err(err) => {
            let str = err.to_string();


            if get_is_debug() {
                println!("RDK error: {}", str);
            }

            return Err(DabError::Err500(str));
        }
    }
}
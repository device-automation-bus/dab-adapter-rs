pub mod device_telemetry;
pub mod health_check;
pub mod device;
pub mod app_telemetry;
pub mod operations;
pub mod voice;
pub mod system;
pub mod applications;
pub mod input;
pub mod output;
pub mod version;

use surf::{Client};
use futures::executor::block_on;
let host_address = String::from("http://192.168.15.112:9998/jsonrpc");

fn http_post(json_string: String) -> Result <String,String> {

    let client = Client::new();
    let response = block_on(async {
        client
            .post(host_address)
            .body_string(json_string)
            .header("Content-Type", "application/json")
            .await
            .unwrap()
            .body_string()
            .await
    });
    match response {
        Ok(val2) => {
            println!("Sucesso: {}", val2);
            return Ok(val2.to_string());
        }
        Err(err) => {
            println!("Erro: {}", err);
            return Err(err.to_string());
        }
    }
}
use futures::executor::block_on;
use surf::Client;

static mut DEVICE_ADDRESS: String = String::new();

pub fn init(address: String) {
    unsafe {
        DEVICE_ADDRESS.push_str(&address);
    }
}
pub fn http_post(json_string: String) -> Result<String, String> {
    let client = Client::new();
    let rdk_address = unsafe { &DEVICE_ADDRESS };
    let response = block_on(async {
        client
            .post(rdk_address)
            .body_string(json_string)
            .header("Content-Type", "application/json")
            .await
            .unwrap()
            .body_string()
            .await
    });
    match response {
        Ok(val2) => {
            return Ok(val2.to_string());
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

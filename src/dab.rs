pub mod app_telemetry;
pub mod applications;
pub mod device;
pub mod device_telemetry;
pub mod health_check;
pub mod input;
pub mod operations;
pub mod output;
pub mod system;
pub mod version;
pub mod voice;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process, thread, time::Duration};

use paho_mqtt::{
    message::Message, message::MessageBuilder, properties::Properties, properties::PropertyCode,
    Client, ConnectOptionsBuilder, CreateOptionsBuilder,
};

fn subscribe(cli: &Client) -> bool {
    if let Err(e) = cli.subscribe("dab/#", 0) {
        println!("Error subscribing topic: {:?}", e);
        return false;
    }
    return true;
}

fn connect(cli: &Client) -> bool {
    // Connect and wait for it to complete or fail.
    let fail_message = MessageBuilder::new()
        .topic("test")
        .payload("Consumer lost connection")
        .finalize();

    let conn_opts = ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .will_message(fail_message)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        return false;
    }
    return true;
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    appId: Option<String>,
    force: Option<bool>,
    keyCode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SimpleResponse {
    pub status: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
}

fn decode_request(packet: Message) -> String {
    String::from_utf8(packet.payload().to_vec()).unwrap()
}

pub fn start(
    mqtt_host: String,
    mqtt_port: u16,
    mut handlers: HashMap<String, Box<dyn FnMut(String) -> Result<String, String>>>,
) {
    // Connect to the MQTT broker and subscribe to all topics starting with `dab/`
    let create_opts = CreateOptionsBuilder::new()
        .server_uri(mqtt_host + ":" + &mqtt_port.to_string())
        .client_id("client id".to_string())
        .mqtt_version(5)
        .finalize();

    let cli = Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let rx = cli.start_consuming();

    if connect(&cli) == false {
        process::exit(1);
    }

    if subscribe(&cli) == false {
        process::exit(1);
    }

    // Process incoming messages
    println!("Processing requests...");
    for msg_rx in rx.iter() {
        if let Some(packet) = msg_rx {
            let result: String;
            let function_topic = std::string::String::from(packet.topic());
            let rx_properties = packet.properties().clone();
            let msg = decode_request(packet);

            match handlers.get_mut(&function_topic) {
                Some(callback) => {
                    println!("OK: {}", function_topic);
                    // println!("MSG: {}",msg);
                    result = match callback(msg) {
                        Ok(r) => r,
                        Err(e) => serde_json::to_string(&ErrorResponse {
                            status: 500,
                            error: e,
                        })
                        .unwrap(),
                    }
                }
                // If we can't get the proper handler, then this function is not implemented (yet)
                _ => {
                    println!("ERROR: {}", function_topic);
                    result = serde_json::to_string(&SimpleResponse { status: 501 }).unwrap();
                }
            }

            let response_topic = rx_properties.get_string(PropertyCode::ResponseTopic);
            let correlation_data = rx_properties.get_string(PropertyCode::CorrelationData);
            if let Some(r) = response_topic {
                let mut msg_prop = Properties::new();
                if let Some(c) = correlation_data {
                    // Set topic properties
                    if let Err(e) = msg_prop.push_val(PropertyCode::CorrelationData, c) {
                        println!("Error setting Msg Properties: {:?}", e);
                        process::exit(1);
                    }
                }
                // Publish to a topic
                let msg_tx = MessageBuilder::new()
                    .topic(r)
                    .payload(result)
                    .qos(0)
                    .properties(msg_prop)
                    .finalize();
                let tok = cli.publish(msg_tx);
                if let Err(e) = tok {
                    println!("Error sending message: {:?}", e);
                }
            }
        } else if !cli.is_connected() {
            println!("Connection lost. Waiting to retry connection");
            loop {
                thread::sleep(Duration::from_millis(5000));
                if connect(&cli) == false {
                    process::exit(1);
                } else {
                    println!("Successfully reconnected");
                    if subscribe(&cli) == false {
                        process::exit(1);
                    }
                    break;
                }
            }
        }
    }
}

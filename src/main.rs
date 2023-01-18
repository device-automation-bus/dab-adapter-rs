use clap::Parser;
use serde_json::Result;
use std::{
    time::Duration,
    collections::HashMap,
    process,
    thread,
};
use tungstenite;
mod apps;
mod device;
mod input;
mod operations;
mod system;
mod utils;

use paho_mqtt::{
    Client,
    ConnectOptionsBuilder,
    CreateOptionsBuilder,
    message::MessageBuilder,
    message::Message,
    properties::Properties,
    properties::PropertyCode,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opt {
    /// The MQTT broker host name or IP (default: localhost)
    #[clap(short, long, value_parser, value_name = "MQTT_HOST")]
    broker: Option<String>,

    /// The MQTT broker port (default: 1883)
    #[clap(short, long, value_parser, value_name = "MQTT_PORT")]
    port: Option<u16>,

    /// The device host name or IP (default: localhost)
    #[clap(short, long, value_parser, value_name = "DEVICE")]
    device: Option<String>,
}

fn subscribe(cli: &Client) -> bool
{
    if let Err(e) = cli.subscribe("dab/#", 0) {
        println!("Error subscribing topic: {:?}", e);
        return false;
    }
    return true;
}

fn connect(cli: &Client) -> bool
{
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

pub fn main() {
    let opt = Opt::parse();
    let mqtt_host = opt.broker.unwrap_or(String::from("localhost"));
    let mqtt_port = opt.port.unwrap_or(1883);
    let device = opt.device.unwrap_or(String::from("localhost"));
    
    // Connect to RDK REST API server (Device)
    let mut ws = match tungstenite::connect(format!("ws://{}:9998/jsonrpc", device)){
        Ok((ws, _r)) => ws,
        Err(e) => panic!("{}", e),
    };    
    
    // Create a HashMap for all operators
    
    let mut handlers: HashMap<String, Box<dyn FnMut(Message, &mut utils::WsStream) -> Result<String>>> = HashMap::new();
    handlers.insert("dab/operations/list".to_string(), Box::new(operations::list::process));
    handlers.insert("dab/applications/list".to_string(), Box::new(apps::list::process));
    handlers.insert("dab/applications/launch".to_string(), Box::new(apps::launch::process));
    handlers.insert("dab/applications/get-state".to_string(), Box::new(apps::get_state::process));
    handlers.insert("dab/applications/exit".to_string(), Box::new(apps::exit::process));
    handlers.insert("dab/device/info".to_string(), Box::new(device::info::process));
    handlers.insert("dab/input/key/list".to_string(), Box::new(input::key_list::process));
    handlers.insert("dab/input/key-press".to_string(), Box::new(input::key_press::process));
    handlers.insert("dab/system/restart".to_string(), Box::new(system::restart::process));
    handlers.insert("dab/system/settings/list".to_string(), Box::new(system::settings_list::process));
    handlers.insert("dab/health-check/get".to_string(), Box::new(utils::health_check::process));
    handlers.insert("dab/version".to_string(), Box::new(utils::version::process));
    handlers.insert("dab/system/language/list".to_string(), Box::new(system::language_list::process));
    handlers.insert("dab/system/language/get".to_string(), Box::new(system::language_get::process));
    handlers.insert("dab/system/language/set".to_string(), Box::new(system::language_set::process));

    // Connect to the MQTT broker and subscribe to all topics starting with `dab/`
    let create_opts = CreateOptionsBuilder::new()
        .server_uri(mqtt_host+":"+&mqtt_port.to_string())
        .client_id("client id".to_string())
        .mqtt_version(5)
        .finalize();

    let cli = Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let rx = cli.start_consuming();

    if connect(&cli)==false {
        process::exit(1);
    }

    if subscribe(&cli)==false {
        process::exit(1);
    }

    // Process incoming messages
    println!("Processing requests...");
    for msg_rx in rx.iter() {
        if let Some(msg) = msg_rx {
            let result: String;
            let function_topic = std::string::String::from(msg.topic());
            let rx_properties = msg.properties().clone();

            match handlers.get_mut(&function_topic) {
                Some(callback) => {
                    println!("OK: {}",function_topic);
                    // println!("MSG: {}",msg);
                    result = match callback(msg, &mut ws) {
                        Ok(r) => r,
                        Err(e) => match utils::dab::respond_error(500, e.to_string()) {
                            Ok(r) => r,
                            Err(e) => e.to_string(),
                        },
                    }
                },
                // If we can't get the proper handler, then this function is not implemented (yet)
                _ => {
                    println!("ERROR: {}",function_topic);
                    result = match utils::dab::respond_not_implemented() {
                        Ok(r) => r,
                        Err(e) => e.to_string(),
                    }
                }
            }
            let response_topic = rx_properties.get_string(PropertyCode::ResponseTopic);
            let correlation_data = rx_properties.get_string(PropertyCode::CorrelationData);
            if let Some(r) = response_topic{
                let mut msg_prop = Properties::new();
                if let Some(c) = correlation_data{
                    // Set topic properties
                    if let Err(e) = msg_prop.push_val(PropertyCode::CorrelationData,c){
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
        }
        else if !cli.is_connected() {
            println!("Connection lost. Waiting to retry connection");
            loop {
                thread::sleep(Duration::from_millis(5000));
                if connect(&cli)==false {
                    process::exit(1);
                } else{
                    println!("Successfully reconnected");
                    if subscribe(&cli)==false {
                        process::exit(1);
                    }
                    break;          
                }
            }
        }
    }
}

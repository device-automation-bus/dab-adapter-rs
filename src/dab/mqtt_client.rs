use crossbeam::channel::{self, Receiver, Sender};
use paho_mqtt as mqtt;
use paho_mqtt::properties::PropertyCode;
use std::thread;

#[derive(Debug)]
pub struct MqttMessage {
    pub function_topic: String,
    pub response_topic: String,
    pub correlation_data: Vec<u8>,
    pub payload: String,
}

#[derive(Clone)]
pub struct MqttClient {
    paho_client: mqtt::Client,
    ipc_channel: (Sender<MqttMessage>, Receiver<MqttMessage>),
    paho_receiver: mqtt::Receiver<Option<mqtt::Message>>,
}

impl MqttClient {
    pub fn new(broker: String, mqtt_port: u16) -> MqttClient {
        // Create a client & define connect options
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(broker + ":" + &mqtt_port.to_string())
            .mqtt_version(5)
            .finalize();

        let paho_client = mqtt::Client::new(create_opts).unwrap();

        let conn_opts = mqtt::ConnectOptionsBuilder::new_v5()
            .keep_alive_interval(std::time::Duration::from_secs(20))
            .clean_start(true)
            .finalize();

        // Connect and wait for it to complete or fail
        if let Err(e) = paho_client.connect(conn_opts) {
            println!("Error connecting: {:?}", e);
        }

        let paho_receiver = paho_client.start_consuming();
        let ipc_channel = channel::unbounded();

        MqttClient {
            paho_client,
            ipc_channel,
            paho_receiver,
        }
    }

    pub fn start(&mut self) {
        // Start another thread to process messages to be sent
        let paho_client = self.paho_client.clone();
        let channel_receiver = self.ipc_channel.1.clone();
        thread::spawn(move || loop {
            let msg_tx = match channel_receiver.recv() {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Error channel tx: {:?}", e);
                    return;
                }
            };
            let function_topic = msg_tx.function_topic;
            let correlation_data = msg_tx.correlation_data;
            let payload = msg_tx.payload;
            let mut msg_prop = mqtt::properties::Properties::new();
            if let Err(e) = msg_prop.push_val(PropertyCode::CorrelationData, correlation_data) {
                println!("Error setting correlation data: {:?}", e);
            }
            let message = mqtt::MessageBuilder::new()
                .topic(function_topic)
                .payload(payload)
                .qos(1)
                .properties(msg_prop)
                .finalize();

            if let Err(e) = paho_client.publish(message) {
                println!("Error sending message: {:?}", e);
            }
        });

        println!("Ready to process DAB requests");
    }
    pub fn subscribe(&mut self, topic: String) {
        let qos = 1;
        if let Err(e) = self.paho_client.subscribe(&topic, qos) {
            println!("Error subscribing to topic: {:?}", e);
        }
    }
    pub fn publish(&self, msg_tx: MqttMessage) {
        self.ipc_channel.0.send(msg_tx).unwrap();
    }
    pub fn receive(&mut self) -> Result<MqttMessage, Option<String>> {
        match self.paho_receiver.recv() {
            Ok(Some(packet)) => {
                let function_topic = std::string::String::from(packet.topic());
                let v: Vec<&str> = function_topic.split('/').collect();
                let operator = v.get(2).unwrap_or(&"");
                if operator == &"messages" {
                    return Err(None);
                }

                let payload_str = packet.payload_str();
                match packet.properties().get_string(PropertyCode::ResponseTopic) {
                    Some(topic) => {
                        let correlation_data = match packet
                            .properties()
                            .get_binary(PropertyCode::CorrelationData)
                        {
                            Some(data) => data,
                            None => {
                                let mut v: Vec<u8> = Vec::new();
                                v.push(0);
                                v
                            }
                        };

                        let rx_msg = MqttMessage {
                            function_topic: function_topic,
                            response_topic: topic,
                            correlation_data: correlation_data,
                            payload: payload_str.to_string(),
                        };
                        Ok(rx_msg)
                    }
                    None => Err(Some("No ResponseTopic provided".to_string())),
                }
            }
            Ok(None) => Err(None),
            Err(e) => Err(Some(e.to_string())),
        }
    }
}

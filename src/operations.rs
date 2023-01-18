use std::net::TcpStream;
use tungstenite::{WebSocket, stream::MaybeTlsStream};
pub type WsStream = WebSocket<MaybeTlsStream<TcpStream>>;

pub mod list {
    use paho_mqtt::message::Message;
    use serde_json::Result;

    mod dab {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub status: u16,
            pub operators: Vec<String>,
        }
    }

    pub fn process(_packet: Message, _ws: &mut super::WsStream) -> Result<String> {
        let mut response = dab::Response {
            status: 200,
            operators: Vec::with_capacity(100),
        };
        response.operators.push("operations/list".to_string());
        response.operators.push("applications/list".to_string());
        response.operators.push("applications/launch".to_string());
        response.operators.push("applications/launch-with-content".to_string());
        response.operators.push("applications/get-state".to_string());
        response.operators.push("applications/exit".to_string());
        response.operators.push("device/info".to_string());
        response.operators.push("system/restart".to_string());
        response.operators.push("system/settings/list".to_string());
        response.operators.push("system/settings/get".to_string());
        response.operators.push("system/settings/set".to_string());
        response.operators.push("input/key/list".to_string());
        response.operators.push("input/key-press".to_string());
        response.operators.push("input/long-key-press".to_string());
        response.operators.push("output/image".to_string());
        response.operators.push("device-telemetry/start".to_string());
        response.operators.push("device-telemetry/stop".to_string());
        response.operators.push("app-telemetry/start".to_string());
        response.operators.push("app-telemetry/stop".to_string());
        response.operators.push("health-check/get".to_string());
        response.operators.push("voice/list".to_string());
        response.operators.push("voice/set".to_string());
        response.operators.push("voice/send-audio".to_string());
        response.operators.push("voice/send-text".to_string());
        response.operators.push("version".to_string());
        response.operators.push("system/language/list".to_string());
        response.operators.push("system/language/get".to_string());
        response.operators.push("system/language/set".to_string());
        response.operators.shrink_to_fit();
        serde_json::to_string(&response)
    }
}
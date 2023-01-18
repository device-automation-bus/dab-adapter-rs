use std::net::TcpStream;
use tungstenite::{WebSocket, stream::MaybeTlsStream};
pub type WsStream = WebSocket<MaybeTlsStream<TcpStream>>;

pub mod restart {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod rpc {
        use serde::{Deserialize, Serialize};
        use crate::utils;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
            pub reason: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Request{
            pub jsonrpc: String,
            pub id: u64,
            pub method: String,
            pub params: Params,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Result {
            pub IARM_Bus_Call_STATUS: Option<u64>,
            pub success: bool,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<Result>,
            pub error: Option<utils::rpc::SimpleError>,
        }
        
        impl utils::Response for Response {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }
    }

    pub fn process(_packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        let request = rpc::Request {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.System.reboot".to_string(),
            params: rpc::Params {
                reason: "DAB_RESTART_REQUEST".to_string(),
            },
        };

        let mut r = String::new();
        utils::rpc::call_and_respond::<rpc::Request, rpc::Response>(request, &mut r, ws)
    }
}

pub mod language_set {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod rpc {
        use serde::{Deserialize, Serialize};
        use crate::utils;

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct ParamsActivate {
            pub callsign: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
            pub ui_language: String,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct RequestActivate{
            pub jsonrpc: String,
            pub id: u64,
            pub method: String,
            pub params: ParamsActivate,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Request{
            pub jsonrpc: String,
            pub id: u64,
            pub method: String,
            pub params: Params,
        }
        
        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct ResultActivate {
            pub success: bool,
        }
        
        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Result {
            pub success: bool,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct ResponseActivate {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<ResultActivate>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<Result>,
        }
        
        impl utils::Response for ResponseActivate {
            fn is_success(&self) -> bool {
                true
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }
        impl utils::Response for Response {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                "unknown error".to_string()
            }
        }
    }
    fn activate_controller(ws: &mut utils::WsStream) -> Result<()> {
        let request = rpc::RequestActivate {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "Controller.1.activate".to_string(),
            params: rpc::ParamsActivate {
                callsign: "org.rdk.UserPreferences".to_string(),
            },
        };
        let mut r = String::new();
        match utils::rpc::call::<rpc::RequestActivate, rpc::ResponseActivate>(request, &mut r, ws){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
    
    pub fn process(_packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        if let Err(e) = activate_controller(ws) {
            return Err(e);
        }
        
        let request = rpc::Request {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.UserPreferences.1.setUILanguage".to_string(),
            params: rpc::Params {
                ui_language: "US_en".to_string(),
            },
        };
        let mut r = String::new();
        utils::rpc::call_and_respond::<rpc::Request, rpc::Response>(request, &mut r, ws)
    }
}

pub mod language_get {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod rpc {
        use serde::{Deserialize, Serialize};
        use crate::utils;

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct ParamsActivate {
            pub callsign: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
            pub ui_language: String,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct RequestActivate{
            pub jsonrpc: String,
            pub id: u64,
            pub method: String,
            pub params: ParamsActivate,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Request{
            pub jsonrpc: String,
            pub id: u64,
            pub method: String,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct ResultActivate {
            pub ui_language: String,
            pub success: bool,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Result {
            pub ui_language: String,
            pub success: bool,
        }

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct ResponseActivate {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<ResultActivate>,
            pub error: Option<utils::rpc::SimpleError>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub jsonrpc: String,
            pub id: u64,
            pub result: Option<Result>,
        }
        
        impl utils::Response for ResponseActivate {
            fn is_success(&self) -> bool {
                true
            }

            fn error_message(&self) -> String {
                match &self.error {
                    Some(e) => e.message.clone(),
                    _ => "unknown error".to_string(),
                }
            }
        }
        impl utils::Response for Response {
            fn is_success(&self) -> bool {
                match &self.result {
                    Some(r) => r.success,
                    _ => false,
                }
            }

            fn error_message(&self) -> String {
                "unknown error".to_string()
            }
        }
    }
    
    mod dab {
        use serde::{Deserialize, Serialize};

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct Response {
            pub status: u16,
            pub language: String,
        }
    }
    
    fn activate_controller(ws: &mut utils::WsStream) -> Result<()> {
        let request = rpc::RequestActivate {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "Controller.1.activate".to_string(),
            params: rpc::ParamsActivate {
                callsign: "org.rdk.UserPreferences".to_string(),
            },
        };
        let mut r = String::new();
        match utils::rpc::call::<rpc::RequestActivate, rpc::ResponseActivate>(request, &mut r, ws){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
    
    pub fn process(_packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        if let Err(e) = activate_controller(ws) {
            return Err(e);
        }
        
        let request = rpc::Request {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.UserPreferences.1.getUILanguage".to_string(),
        };
        let mut r = String::new();
        match utils::rpc::call::<rpc::Request, rpc::Response>(request, &mut r, ws){
            Ok(response) => {
                let language = response.result.unwrap().ui_language;
                serde_json::to_string(&dab::Response { status: 200, language: language })
            },
            Err(e) => Err(e)
        }
    }
}

pub mod language_list {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod dab {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub status: u16,
            pub languages: Vec<String>,
        }
    }

    pub fn process(_packet: Message, _ws: &mut super::WsStream) -> Result<String> {
        /*
            Note: As defined on the org.rdk.UserPreferences plugin documentation
            (https://rdkcentral.github.io/rdkservices/#/api/UserPreferencesPlugin):
            "The language is written to the /opt/user_preferences.conf file on the device. 
            It is the responsibility of the client application to validate the language value and process 
            it if required. Any language string that is valid on the client can be set" 
        */
        // response.language = utils::get_rfc_5646_language_tag();
        // response.language.shrink_to_fit();
        let response = dab::Response {
            status: 200,
            languages: utils::get_rfc_5646_language_tag()
        };
        serde_json::to_string(&response)
    }
}

pub mod settings_list {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod dab {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub status: u16,
            pub language: Vec<String>,
        }
    }

    pub fn process(_packet: Message, _ws: &mut super::WsStream) -> Result<String> {
        /*
            Note: As defined on the org.rdk.UserPreferences plugin documentation
            (https://rdkcentral.github.io/rdkservices/#/api/UserPreferencesPlugin):
            "The language is written to the /opt/user_preferences.conf file on the device. 
            It is the responsibility of the client application to validate the language value and process 
            it if required. Any language string that is valid on the client can be set" 
        */
        // response.language = utils::get_rfc_5646_language_tag();
        // response.language.shrink_to_fit();
        let response = dab::Response {
            status: 200,
            language: utils::get_rfc_5646_language_tag()
        };
        serde_json::to_string(&response)
    }
}
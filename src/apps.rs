// All apps-related RPC requests need similar (if not identical)
// parameters, so let's implement such a structure as a top-level
// module other sub-modules can refer to.
mod rpc {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Params {
        pub callsign: String,
        pub r#type: Option<String>,
        pub uri: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request{
        pub jsonrpc: String,
        pub id: u64,
        pub method: String,
        pub params: Params,
    }
}

pub mod list {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod rpc {
        use serde::{Deserialize, Serialize};
        use crate::utils;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Result {
            pub types: Option<Vec<String>>,
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

    mod dab {
        use serde::{Deserialize, Serialize};

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Item {
            pub appId: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub status: u16,
            pub applications: Vec<Item>,
        }
    }

    pub fn process(_packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        let request = utils::rpc::SimpleRequest {
            jsonrpc: "2.0".to_string(),
            id: utils::get_request_id(),
            method: "org.rdk.RDKShell.getAvailableTypes".to_string(),
        };

        let mut r = String::new();
        match utils::rpc::call::<utils::rpc::SimpleRequest, rpc::Response>(request, &mut r, ws) {
            Ok(rpc_response) => {
                let result = rpc_response.result.unwrap();
                let dab_response = match result.types {
                    // Available applications are listed as an array of strings,
                    // each entry being the appId of a single app
                    Some(apps) => {
                        let mut response = dab::Response {
                            status: 200,
                            applications: Vec::with_capacity(apps.len()),
                        };
                        for app in apps {
                            response.applications.push(dab::Item { appId: app });
                        }
                        response
                    },
                    None => dab::Response {
                        status: 200,
                        applications: vec![],
                    },
                };
                serde_json::to_string(&dab_response)
            },
            Err(e) => Err(e)
        }
    }
}

pub mod launch {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod rpc {
        use serde::{Deserialize, Serialize};
        use crate::utils;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Result {
            pub launchtype: Option<String>,
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

    pub fn process(packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        match utils::dab::decode_request(packet) {
            Ok(payload) => {
                if payload.appId.is_none() {
                    utils::dab::respond_error(400, "request missing 'appId' parameter".to_string())
                } else {
                    let app_id = payload.appId.unwrap();
                    let request = super::rpc::Request {
                        jsonrpc: "2.0".to_string(),
                        id: utils::get_request_id(),
                        method: "org.rdk.RDKShell.launch".to_string(),
                        params: super::rpc::Params {
                            callsign: app_id.to_lowercase(),
                            r#type: Some(app_id),
                            uri: None,
                        },
                    };

                    let mut r = String::new();
                    utils::rpc::call_and_respond::<super::rpc::Request, rpc::Response>(request, &mut r, ws)
                }
            },
            Err(e) => Err(e)
        }
    }
}

pub mod get_state {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    mod rpc {
        use serde::{Deserialize, Serialize};
        use crate::utils;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct State {
            pub callsign: String,
            pub state: String,
            pub uri: String,
        }
        
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Result {
            pub state: Option<Vec<State>>,
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

    mod dab {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub status: u16,
            pub state: String,
        }
    }

    pub fn process(packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        match utils::dab::decode_request(packet) {
            Ok(payload) => {
                if payload.appId.is_none() {
                    utils::dab::respond_error(400, "request missing 'appId' parameter".to_string())
                } else {
                    let app_id = payload.appId.unwrap().to_lowercase();
                    let request = utils::rpc::SimpleRequest {
                        jsonrpc: "2.0".to_string(),
                        id: utils::get_request_id(),
                        method: "org.rdk.RDKShell.getState".to_string(),
                    };

                    let mut r = String::new();
                    match utils::rpc::call::<utils::rpc::SimpleRequest, rpc::Response>(request, &mut r, ws) {
                        Ok(response) => {
                            let result = response.result.unwrap();
                            let state = match result.state {
                                // getState returns the state of all currently running apps,
                                // so we must filter according to the requested appId
                                Some(s) => {
                                    let mut cur_state = String::new();
                                    for item in s {
                                        if item.callsign != app_id {
                                            continue;
                                        }
        
                                        match item.state.as_str() {
                                            "suspended" => cur_state.push_str("BACKGROUND"),
                                            _ => cur_state.push_str("FOREGROUND"),
                                        }

                                        break;
                                    }

                                    // We couldn't find the requested appId in the list, that
                                    // means the app isn't running yet
                                    if cur_state.is_empty() {
                                        cur_state.push_str("STOPPED");
                                    }
                                    cur_state
                                },
                                None => "STOPPED".to_string(),
                            };

                            serde_json::to_string(&dab::Response { status: 200, state })
                        },
                        Err(e) => Err(e)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

pub mod exit {
    use paho_mqtt::message::Message;
    use serde_json::Result;
    use crate::utils;

    pub fn process(packet: Message, ws: &mut utils::WsStream) -> Result<String> {
        match utils::dab::decode_request(packet) {
            Ok(payload) => {
                if payload.appId.is_none() {
                    utils::dab::respond_error(400, "request missing 'appId' parameter".to_string())
                } else {
                    let app_id = payload.appId.unwrap();
                    let method = match payload.force {
                        Some(true) => "org.rdk.RDKShell.destroy",
                        _ => "org.rdk.RDKShell.suspend",
                    };

                    let request = super::rpc::Request {
                        jsonrpc: "2.0".to_string(),
                        id: utils::get_request_id(),
                        method: method.to_string(),
                        params: super::rpc::Params {
                            callsign: app_id.to_lowercase(),
                            r#type: None,
                            uri: None,
                        },
                    };

                    let mut r = String::new();
                    utils::rpc::call_and_respond::<super::rpc::Request, utils::rpc::SimpleResponse>(request, &mut r, ws)
                }
            },
            Err(e) => Err(e)
        }
    }
}

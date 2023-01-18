use std::net::TcpStream;
use tungstenite::{WebSocket, stream::MaybeTlsStream};

pub type WsStream = WebSocket<MaybeTlsStream<TcpStream>>;

// This trait allows us to provide a standard way to
// query the state and (if applicable) error message of
// an RPC request, as all response structs have different
// structures and cannot be used/extended as we do with C
// pointers.
pub trait Response {
    fn is_success(&self) -> bool;
    fn error_message(&self) -> String;
}

// We need a unique ID for each request, this is the simple
// (and probably bad) way to do so
// TODO: implement this the "right" way
static mut ID: u64 = 0;

pub fn get_request_id() -> u64 {
    unsafe {
        ID = ID + 1;
        ID
    }
}

pub mod rpc {
    use serde::{Serialize, Deserialize, de::Error};
    use serde_json::Result;
    use tungstenite::protocol::Message;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SimpleRequest{
        pub jsonrpc: String,
        pub id: u64,
        pub method: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SimpleError {
        pub success: bool,
        pub code: u32,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SimpleResult {
        pub success: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SimpleResponse {
        pub jsonrpc: String,
        pub id: u64,
        pub result: Option<SimpleResult>,
        pub error: Option<SimpleError>,
    }

    // TODO: we should probably use a "blanket" implementation or allowing
    // to derive from the trait: this would avoid having to reimplement
    // this trait in the same exact way for all relevant structures.
    // I just don't know how to do it (yet).
    impl super::Response for SimpleResponse {
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

    // Send the RPC request and return the response as a string
    // This function is generic, with T being the request struct type
    pub fn call_raw<T>(request: T, ws: &mut super::WsStream) -> Result<String>
        where T: Serialize
    {
        let message = Message::Text(serde_json::to_string(&request).unwrap());
        if ws.write_message(message).is_err()
        {
            return Err(serde_json::Error::custom("unable to send RPC request to device"));
        }

        match ws.read_message() {
            Ok(Message::Text(m)) => Ok(m),
            Ok(_) => Err(serde_json::Error::custom("non-text response received")),
            Err(e) => Err(serde_json::Error::custom(format!("WebSocket error: {}", e))),
        }
    }

    // Send the RPC request and return the response as a struct
    // This function is generic:
    //   * T is the request struct type
    //   * U is the response struct type
    pub fn call<'a, T, U>(request: T, response: &'a mut String, ws: &mut super::WsStream) -> Result<U>
        where T: Serialize,
              U: Deserialize<'a> + super::Response
    {
        match call_raw(request, ws) {
            Ok(m) => {
                response.clear();
                response.push_str(m.as_str());
                match serde_json::from_str::<U>(response.as_str()) {
                    Ok(result) => {
                        if result.is_success() {
                            Ok(result)
                        } else {
                            Err(serde_json::Error::custom(format!("RPC request failed: {}", result.error_message())))
                        }
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    // Send the RPC request and return a simple DAB "success" (or error) message
    // This function is generic:
    //   * T is the request struct type
    //   * U is the response struct type
    pub fn call_and_respond<'a, T, U>(request: T, response: &'a mut String, ws: &mut super::WsStream) -> Result<String>
        where T: Serialize,
              U: Deserialize<'a> + super::Response
    {
        match call::<T, U>(request, response, ws) {
            Ok(_) => super::dab::respond_success(),
            Err(e) => Err(e),
        }
    }
}

pub mod dab {
    use serde::{Serialize, Deserialize, de::Error};
    use serde_json::Result;
    use paho_mqtt::message::Message;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub appId: Option<String>,
        pub force: Option<bool>,
        pub keyCode:Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SimpleResponse {
        pub status: u16,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ErrorResponse {
        pub status: u16,
        pub error: String,
    }

    pub fn decode_request(packet: Message) -> Result<Request> {
        if let Ok(payload) = String::from_utf8(packet.payload().to_vec()) {
            serde_json::from_str(payload.as_str())
        } else {
            Err(serde_json::Error::custom("unable to decode DAB request"))
        }
    }

    pub fn respond_with_code(status: u16) -> Result<String> {
        serde_json::to_string(&SimpleResponse { status })
    }

    pub fn respond_success() -> Result<String> {
        respond_with_code(200)
    }

    pub fn respond_error(status: u16, error: String) -> Result<String> {
        serde_json::to_string(&ErrorResponse { status, error })
    }

    pub fn respond_not_implemented() -> Result<String> {
        respond_with_code(501)
    }
}

pub mod health_check {
    use paho_mqtt::message::Message;
    use serde_json::Result;

    pub fn process(_packet: Message, _ws: &mut super::WsStream) -> Result<String> {
        // Simple health check, nothing expected but a "success" response
        super::dab::respond_success()
    }
}

pub mod version {
    use paho_mqtt::message::Message;
    use serde_json::Result;

    mod dab {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Response {
            pub status: u16,
            pub versions: Vec<String>,
        }
    }

    pub fn process(_packet: Message, _ws: &mut super::WsStream) -> Result<String> {
        // We only support DAB 1.0 for now
        serde_json::to_string(&dab::Response { 
            status: 200,
            versions: vec![String::from("2.0")] 
        })
    }
}

// Return language tags defined in RFC 5646.
/*
    Note: As defined on the org.rdk.UserPreferences plugin documentation
    (https://rdkcentral.github.io/rdkservices/#/api/UserPreferencesPlugin):
    "The language is written to the /opt/user_preferences.conf file on the device. 
    It is the responsibility of the client application to validate the language value and process 
    it if required. Any language string that is valid on the client can be set" 
*/
pub fn get_rfc_5646_language_tag() -> Vec<String>{
    let rfc_5646_language_tag = vec![
      "af-NA".to_string(),
      "af-ZA".to_string(),
      "agq-CM".to_string(),
      "ak-GH".to_string(),
      "am-ET".to_string(),
      "ar-001".to_string(),
      "ar-AE".to_string(),
      "ar-BH".to_string(),
      "ar-DJ".to_string(),
      "ar-DZ".to_string(),
      "ar-EG".to_string(),
      "ar-EH".to_string(),
      "ar-ER".to_string(),
      "ar-IL".to_string(),
      "ar-IQ".to_string(),
      "ar-JO".to_string(),
      "ar-KM".to_string(),
      "ar-KW".to_string(),
      "ar-LB".to_string(),
      "ar-LY".to_string(),
      "ar-MA".to_string(),
      "ar-MR".to_string(),
      "ar-OM".to_string(),
      "ar-PS".to_string(),
      "ar-QA".to_string(),
      "ar-SA".to_string(),
      "ar-SD".to_string(),
      "ar-SO".to_string(),
      "ar-SS".to_string(),
      "ar-SY".to_string(),
      "ar-TD".to_string(),
      "ar-TN".to_string(),
      "ar-YE".to_string(),
      "arn-CL".to_string(),
      "as-IN".to_string(),
      "asa-TZ".to_string(),
      "ast-ES".to_string(),
      "az-AZ".to_string(),
      "az-AZ".to_string(),
      "ba-RU".to_string(),
      "bas-CM".to_string(),
      "be-BY".to_string(),
      "bem-ZM".to_string(),
      "bez-TZ".to_string(),
      "bg-BG".to_string(),
      "bm-ML".to_string(),
      "bn-BD".to_string(),
      "bn-IN".to_string(),
      "bo-CN".to_string(),
      "bo-IN".to_string(),
      "br-FR".to_string(),
      "brx-IN".to_string(),
      "bs-BA".to_string(),
      "bs-BA".to_string(),
      "byn-ER".to_string(),
      "ca-AD".to_string(),
      "ca-ES".to_string(),
      "ca-FR".to_string(),
      "ca-IT".to_string(),
      "ccp-BD".to_string(),
      "ccp-IN".to_string(),
      "ce-RU".to_string(),
      "ceb-PH".to_string(),
      "cgg-UG".to_string(),
      "chr-US".to_string(),
      "ckb-IQ".to_string(),
      "ckb-IR".to_string(),
      "co-FR".to_string(),
      "cs-CZ".to_string(),
      "cv-RU".to_string(),
      "cy-GB".to_string(),
      "da-DK".to_string(),
      "da-GL".to_string(),
      "dav-KE".to_string(),
      "de-AT".to_string(),
      "de-BE".to_string(),
      "de-CH".to_string(),
      "de-DE".to_string(),
      "de-IT".to_string(),
      "de-LI".to_string(),
      "de-LU".to_string(),
      "dje-NE".to_string(),
      "dsb-DE".to_string(),
      "dua-CM".to_string(),
      "dv-MV".to_string(),
      "dyo-SN".to_string(),
      "dz-BT".to_string(),
      "ebu-KE".to_string(),
      "ee-GH".to_string(),
      "ee-TG".to_string(),
      "el-CY".to_string(),
      "el-GR".to_string(),
      "en-001".to_string(),
      "en-150".to_string(),
      "en-AD".to_string(),
      "en-AE".to_string(),
      "en-AG".to_string(),
      "en-AI".to_string(),
      "en-AL".to_string(),
      "en-AR".to_string(),
      "en-AS".to_string(),
      "en-AT".to_string(),
      "en-AU".to_string(),
      "en-BA".to_string(),
      "en-BB".to_string(),
      "en-BD".to_string(),
      "en-BE".to_string(),
      "en-BG".to_string(),
      "en-BI".to_string(),
      "en-BM".to_string(),
      "en-BR".to_string(),
      "en-BS".to_string(),
      "en-BW".to_string(),
      "en-BZ".to_string(),
      "en-CA".to_string(),
      "en-CC".to_string(),
      "en-CH".to_string(),
      "en-CK".to_string(),
      "en-CL".to_string(),
      "en-CM".to_string(),
      "en-CN".to_string(),
      "en-CO".to_string(),
      "en-CX".to_string(),
      "en-CY".to_string(),
      "en-CZ".to_string(),
      "en-DE".to_string(),
      "en-DG".to_string(),
      "en-DK".to_string(),
      "en-DM".to_string(),
      "en-EE".to_string(),
      "en-ER".to_string(),
      "en-ES".to_string(),
      "en-FI".to_string(),
      "en-FJ".to_string(),
      "en-FK".to_string(),
      "en-FM".to_string(),
      "en-FR".to_string(),
      "en-GB".to_string(),
      "en-GD".to_string(),
      "en-GG".to_string(),
      "en-GH".to_string(),
      "en-GI".to_string(),
      "en-GM".to_string(),
      "en-GR".to_string(),
      "en-GU".to_string(),
      "en-GY".to_string(),
      "en-HK".to_string(),
      "en-HR".to_string(),
      "en-HU".to_string(),
      "en-ID".to_string(),
      "en-IE".to_string(),
      "en-IL".to_string(),
      "en-IM".to_string(),
      "en-IN".to_string(),
      "en-IO".to_string(),
      "en-IS".to_string(),
      "en-IT".to_string(),
      "en-JE".to_string(),
      "en-JM".to_string(),
      "en-JP".to_string(),
      "en-KE".to_string(),
      "en-KI".to_string(),
      "en-KN".to_string(),
      "en-KR".to_string(),
      "en-KY".to_string(),
      "en-LC".to_string(),
      "en-LR".to_string(),
      "en-LS".to_string(),
      "en-LT".to_string(),
      "en-LU".to_string(),
      "en-LV".to_string(),
      "en-ME".to_string(),
      "en-MG".to_string(),
      "en-MH".to_string(),
      "en-MM".to_string(),
      "en-MO".to_string(),
      "en-MP".to_string(),
      "en-MS".to_string(),
      "en-MT".to_string(),
      "en-MU".to_string(),
      "en-MV".to_string(),
      "en-MW".to_string(),
      "en-MX".to_string(),
      "en-MY".to_string(),
      "en-NA".to_string(),
      "en-NF".to_string(),
      "en-NG".to_string(),
      "en-NL".to_string(),
      "en-NO".to_string(),
      "en-NR".to_string(),
      "en-NU".to_string(),
      "en-NZ".to_string(),
      "en-PG".to_string(),
      "en-PH".to_string(),
      "en-PK".to_string(),
      "en-PL".to_string(),
      "en-PN".to_string(),
      "en-PR".to_string(),
      "en-PT".to_string(),
      "en-PW".to_string(),
      "en-RO".to_string(),
      "en-RS".to_string(),
      "en-RU".to_string(),
      "en-RW".to_string(),
      "en-SA".to_string(),
      "en-SB".to_string(),
      "en-SC".to_string(),
      "en-SD".to_string(),
      "en-SE".to_string(),
      "en-SG".to_string(),
      "en-SH".to_string(),
      "en-SI".to_string(),
      "en-SK".to_string(),
      "en-SL".to_string(),
      "en-SS".to_string(),
      "en-SX".to_string(),
      "en-SZ".to_string(),
      "en-TC".to_string(),
      "en-TH".to_string(),
      "en-TK".to_string(),
      "en-TO".to_string(),
      "en-TR".to_string(),
      "en-TT".to_string(),
      "en-TV".to_string(),
      "en-TW".to_string(),
      "en-TZ".to_string(),
      "en-UA".to_string(),
      "en-UG".to_string(),
      "en-UM".to_string(),
      "en-US".to_string(),
      "en-US".to_string(),
      "en-VC".to_string(),
      "en-VG".to_string(),
      "en-VI".to_string(),
      "en-VU".to_string(),
      "en-WS".to_string(),
      "en-ZA".to_string(),
      "en-ZM".to_string(),
      "en-ZW".to_string(),
      "eo-001".to_string(),
      "es-419".to_string(),
      "es-AG".to_string(),
      "es-AI".to_string(),
      "es-AR".to_string(),
      "es-AW".to_string(),
      "es-BB".to_string(),
      "es-BL".to_string(),
      "es-BM".to_string(),
      "es-BO".to_string(),
      "es-BQ".to_string(),
      "es-BR".to_string(),
      "es-BS".to_string(),
      "es-BZ".to_string(),
      "es-CA".to_string(),
      "es-CL".to_string(),
      "es-CO".to_string(),
      "es-CR".to_string(),
      "es-CU".to_string(),
      "es-CW".to_string(),
      "es-DM".to_string(),
      "es-DO".to_string(),
      "es-EA".to_string(),
      "es-EC".to_string(),
      "es-ES".to_string(),
      "es-FK".to_string(),
      "es-GD".to_string(),
      "es-GF".to_string(),
      "es-GL".to_string(),
      "es-GP".to_string(),
      "es-GQ".to_string(),
      "es-GT".to_string(),
      "es-GY".to_string(),
      "es-HN".to_string(),
      "es-HT".to_string(),
      "es-IC".to_string(),
      "es-KN".to_string(),
      "es-KY".to_string(),
      "es-LC".to_string(),
      "es-MF".to_string(),
      "es-MQ".to_string(),
      "es-MS".to_string(),
      "es-MX".to_string(),
      "es-NI".to_string(),
      "es-PA".to_string(),
      "es-PE".to_string(),
      "es-PH".to_string(),
      "es-PM".to_string(),
      "es-PR".to_string(),
      "es-PY".to_string(),
      "es-SR".to_string(),
      "es-SV".to_string(),
      "es-SX".to_string(),
      "es-TC".to_string(),
      "es-TT".to_string(),
      "es-US".to_string(),
      "es-UY".to_string(),
      "es-VC".to_string(),
      "es-VE".to_string(),
      "es-VG".to_string(),
      "es-VI".to_string(),
      "et-EE".to_string(),
      "eu-ES".to_string(),
      "ewo-CM".to_string(),
      "fa-AF".to_string(),
      "fa-IR".to_string(),
      "ff-BF".to_string(),
      "ff-CM".to_string(),
      "ff-GH".to_string(),
      "ff-GM".to_string(),
      "ff-GN".to_string(),
      "ff-GW".to_string(),
      "ff-LR".to_string(),
      "ff-MR".to_string(),
      "ff-NE".to_string(),
      "ff-NG".to_string(),
      "ff-SL".to_string(),
      "ff-SN".to_string(),
      "fi-FI".to_string(),
      "fil-PH".to_string(),
      "fo-DK".to_string(),
      "fo-FO".to_string(),
      "fr-BE".to_string(),
      "fr-BF".to_string(),
      "fr-BI".to_string(),
      "fr-BJ".to_string(),
      "fr-BL".to_string(),
      "fr-CA".to_string(),
      "fr-CD".to_string(),
      "fr-CF".to_string(),
      "fr-CG".to_string(),
      "fr-CH".to_string(),
      "fr-CI".to_string(),
      "fr-CM".to_string(),
      "fr-DJ".to_string(),
      "fr-DZ".to_string(),
      "fr-FR".to_string(),
      "fr-GA".to_string(),
      "fr-GF".to_string(),
      "fr-GN".to_string(),
      "fr-GP".to_string(),
      "fr-GQ".to_string(),
      "fr-HT".to_string(),
      "fr-KM".to_string(),
      "fr-LU".to_string(),
      "fr-MA".to_string(),
      "fr-MC".to_string(),
      "fr-MF".to_string(),
      "fr-MG".to_string(),
      "fr-ML".to_string(),
      "fr-MQ".to_string(),
      "fr-MR".to_string(),
      "fr-MU".to_string(),
      "fr-NC".to_string(),
      "fr-NE".to_string(),
      "fr-PF".to_string(),
      "fr-PM".to_string(),
      "fr-RE".to_string(),
      "fr-RW".to_string(),
      "fr-SC".to_string(),
      "fr-SN".to_string(),
      "fr-SY".to_string(),
      "fr-TD".to_string(),
      "fr-TG".to_string(),
      "fr-TN".to_string(),
      "fr-VU".to_string(),
      "fr-WF".to_string(),
      "fr-YT".to_string(),
      "fur-IT".to_string(),
      "fy-NL".to_string(),
      "ga-IE".to_string(),
      "gaa-GH".to_string(),
      "gd-GB".to_string(),
      "gez-ER".to_string(),
      "gez-ET".to_string(),
      "gl-ES".to_string(),
      "gn-PY".to_string(),
      "gsw-CH".to_string(),
      "gsw-FR".to_string(),
      "gsw-LI".to_string(),
      "gu-IN".to_string(),
      "guz-KE".to_string(),
      "gv-IM".to_string(),
      "ha-GH".to_string(),
      "ha-NE".to_string(),
      "ha-NG".to_string(),
      "haw-US".to_string(),
      "he-IL".to_string(),
      "hi-IN".to_string(),
      "hr-BA".to_string(),
      "hr-HR".to_string(),
      "hsb-DE".to_string(),
      "hu-HU".to_string(),
      "hy-AM".to_string(),
      "ia-001".to_string(),
      "id-ID".to_string(),
      "ig-NG".to_string(),
      "ii-CN".to_string(),
      "io-001".to_string(),
      "is-IS".to_string(),
      "it-CH".to_string(),
      "it-IT".to_string(),
      "it-SM".to_string(),
      "it-VA".to_string(),
      "iu-CA".to_string(),
      "ja-JP".to_string(),
      "jbo-001".to_string(),
      "jgo-CM".to_string(),
      "jmc-TZ".to_string(),
      "jv-ID".to_string(),
      "ka-GE".to_string(),
      "kab-DZ".to_string(),
      "kaj-NG".to_string(),
      "kam-KE".to_string(),
      "kcg-NG".to_string(),
      "kde-TZ".to_string(),
      "kea-CV".to_string(),
      "khq-ML".to_string(),
      "ki-KE".to_string(),
      "kk-KZ".to_string(),
      "kkj-CM".to_string(),
      "kl-GL".to_string(),
      "kln-KE".to_string(),
      "km-KH".to_string(),
      "kn-IN".to_string(),
      "ko-KP".to_string(),
      "ko-KR".to_string(),
      "kok-IN".to_string(),
      "kpe-GN".to_string(),
      "kpe-LR".to_string(),
      "ks-IN".to_string(),
      "ks-IN".to_string(),
      "ks-IN".to_string(),
      "ksb-TZ".to_string(),
      "ksf-CM".to_string(),
      "ksh-DE".to_string(),
      "ku-TR".to_string(),
      "kw-GB".to_string(),
      "ky-KG".to_string(),
      "lag-TZ".to_string(),
      "lb-LU".to_string(),
      "lg-UG".to_string(),
      "lkt-US".to_string(),
      "ln-AO".to_string(),
      "ln-CD".to_string(),
      "ln-CF".to_string(),
      "ln-CG".to_string(),
      "lo-LA".to_string(),
      "lrc-IQ".to_string(),
      "lrc-IR".to_string(),
      "lt-LT".to_string(),
      "lu-CD".to_string(),
      "luo-KE".to_string(),
      "luy-KE".to_string(),
      "lv-LV".to_string(),
      "mas-KE".to_string(),
      "mas-TZ".to_string(),
      "mer-KE".to_string(),
      "mfe-MU".to_string(),
      "mg-MG".to_string(),
      "mgh-MZ".to_string(),
      "mgo-CM".to_string(),
      "mi-NZ".to_string(),
      "mk-MK".to_string(),
      "ml-IN".to_string(),
      "mn-MN".to_string(),
      "mni-IN".to_string(),
      "mni-IN".to_string(),
      "moh-CA".to_string(),
      "mr-IN".to_string(),
      "ms-BN".to_string(),
      "ms-BN".to_string(),
      "ms-MY".to_string(),
      "ms-MY".to_string(),
      "ms-SG".to_string(),
      "mt-MT".to_string(),
      "mua-CM".to_string(),
      "my-MM".to_string(),
      "myv-RU".to_string(),
      "mzn-IR".to_string(),
      "naq-NA".to_string(),
      "nb-NO".to_string(),
      "nb-SJ".to_string(),
      "nd-ZW".to_string(),
      "nds-DE".to_string(),
      "nds-NL".to_string(),
      "ne-IN".to_string(),
      "ne-NP".to_string(),
      "nl-AW".to_string(),
      "nl-BE".to_string(),
      "nl-BQ".to_string(),
      "nl-CW".to_string(),
      "nl-NL".to_string(),
      "nl-SR".to_string(),
      "nl-SX".to_string(),
      "nmg-CM".to_string(),
      "nn-NO".to_string(),
      "nnh-CM".to_string(),
      "nqo-GN".to_string(),
      "nr-ZA".to_string(),
      "nso-ZA".to_string(),
      "nus-SS".to_string(),
      "ny-MW".to_string(),
      "nyn-UG".to_string(),
      "oc-FR".to_string(),
      "om-ET".to_string(),
      "om-KE".to_string(),
      "or-IN".to_string(),
      "os-GE".to_string(),
      "os-RU".to_string(),
      "pa-IN".to_string(),
      "pa-PK".to_string(),
      "pa-PK".to_string(),
      "pl-PL".to_string(),
      "ps-AF".to_string(),
      "ps-PK".to_string(),
      "pt-AO".to_string(),
      "pt-BR".to_string(),
      "pt-CH".to_string(),
      "pt-CV".to_string(),
      "pt-FR".to_string(),
      "pt-GQ".to_string(),
      "pt-GW".to_string(),
      "pt-LU".to_string(),
      "pt-MO".to_string(),
      "pt-MZ".to_string(),
      "pt-PT".to_string(),
      "pt-ST".to_string(),
      "pt-TL".to_string(),
      "qu-BO".to_string(),
      "qu-EC".to_string(),
      "qu-PE".to_string(),
      "rm-CH".to_string(),
      "rn-BI".to_string(),
      "ro-MD".to_string(),
      "ro-RO".to_string(),
      "rof-TZ".to_string(),
      "ru-BY".to_string(),
      "ru-KG".to_string(),
      "ru-KZ".to_string(),
      "ru-MD".to_string(),
      "ru-RU".to_string(),
      "ru-UA".to_string(),
      "rw-RW".to_string(),
      "rwk-TZ".to_string(),
      "sa-IN".to_string(),
      "sah-RU".to_string(),
      "saq-KE".to_string(),
      "sat-IN".to_string(),
      "sat-IN".to_string(),
      "sbp-TZ".to_string(),
      "sc-IT".to_string(),
      "scn-IT".to_string(),
      "sd-PK".to_string(),
      "se-FI".to_string(),
      "se-NO".to_string(),
      "se-SE".to_string(),
      "seh-MZ".to_string(),
      "ses-ML".to_string(),
      "sg-CF".to_string(),
      "shi-MA".to_string(),
      "shi-MA".to_string(),
      "si-LK".to_string(),
      "sk-SK".to_string(),
      "sl-SI".to_string(),
      "smn-FI".to_string(),
      "sn-ZW".to_string(),
      "so-DJ".to_string(),
      "so-ET".to_string(),
      "so-KE".to_string(),
      "so-SO".to_string(),
      "sq-AL".to_string(),
      "sq-MK".to_string(),
      "sq-XK".to_string(),
      "sr-BA".to_string(),
      "sr-BA".to_string(),
      "sr-ME".to_string(),
      "sr-ME".to_string(),
      "sr-RS".to_string(),
      "sr-RS".to_string(),
      "sr-XK".to_string(),
      "sr-XK".to_string(),
      "ss-SZ".to_string(),
      "ss-ZA".to_string(),
      "st-LS".to_string(),
      "st-ZA".to_string(),
      "sv-AX".to_string(),
      "sv-FI".to_string(),
      "sv-SE".to_string(),
      "sw-CD".to_string(),
      "sw-KE".to_string(),
      "sw-TZ".to_string(),
      "sw-UG".to_string(),
      "syr-IQ".to_string(),
      "syr-SY".to_string(),
      "ta-IN".to_string(),
      "ta-LK".to_string(),
      "ta-MY".to_string(),
      "ta-SG".to_string(),
      "te-IN".to_string(),
      "teo-KE".to_string(),
      "teo-UG".to_string(),
      "tg-TJ".to_string(),
      "th-TH".to_string(),
      "ti-ER".to_string(),
      "ti-ET".to_string(),
      "tig-ER".to_string(),
      "tk-TM".to_string(),
      "tn-BW".to_string(),
      "tn-ZA".to_string(),
      "to-TO".to_string(),
      "tr-CY".to_string(),
      "tr-TR".to_string(),
      "trv-TW".to_string(),
      "ts-ZA".to_string(),
      "tt-RU".to_string(),
      "twq-NE".to_string(),
      "tzm-MA".to_string(),
      "ug-CN".to_string(),
      "uk-UA".to_string(),
      "ur-IN".to_string(),
      "ur-IN".to_string(),
      "ur-IN".to_string(),
      "ur-PK".to_string(),
      "ur-PK".to_string(),
      "ur-PK".to_string(),
      "uz-AF".to_string(),
      "uz-UZ".to_string(),
      "uz-UZ".to_string(),
      "vai-LR".to_string(),
      "vai-LR".to_string(),
      "ve-ZA".to_string(),
      "vi-VN".to_string(),
      "vun-TZ".to_string(),
      "wa-BE".to_string(),
      "wae-CH".to_string(),
      "wal-ET".to_string(),
      "wo-SN".to_string(),
      "xh-ZA".to_string(),
      "xog-UG".to_string(),
      "yav-CM".to_string(),
      "yi-001".to_string(),
      "yo-BJ".to_string(),
      "yo-NG".to_string(),
      "yue-CN".to_string(),
      "yue-HK".to_string(),
      "zgh-MA".to_string(),
      "zh-CN".to_string(),
      "zh-CN".to_string(),
      "zh-HK".to_string(),
      "zh-HK".to_string(),
      "zh-MO".to_string(),
      "zh-MO".to_string(),
      "zh-SG".to_string(),
      "zh-TW".to_string(),
      "zu-ZA".to_string(),
    ];
    rfc_5646_language_tag
}
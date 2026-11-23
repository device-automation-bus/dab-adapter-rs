use crate::dab::structs::Application;
use crate::dab::structs::ApplicationListRequest;
use crate::dab::structs::DabError;
use crate::dab::structs::ListApplicationsResponse;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::RdkResponse;
use serde::Deserialize;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: ApplicationListRequest) -> Result<String, DabError> {
    let mut ResponseOperator = ListApplicationsResponse::default();
    // *** Fill in the fields of the struct Application here ***

    #[derive(Deserialize)]
    struct GetAvailableTypesResult {
        types: Vec<String>,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetAvailableTypesResult> =
        rdk_request("org.rdk.RDKShell.getAvailableTypes")?;
    for s in rdkresponse.result.types.iter() {
        match s.as_str() {
            "YouTube" => {
                let app = Application {
                    appId: ("YouTube").to_string(),
                };
                ResponseOperator.applications.push(app);
            }
            "YouTubeTV" => {
                let app = Application {
                    appId: ("YouTubeTV").to_string(),
                };
                ResponseOperator.applications.push(app);
            }
            "Amazon" => {
                let app = Application {
                    appId: ("PrimeVideo").to_string(),
                };
                ResponseOperator.applications.push(app);
            }
            "Netflix" => {
                let app = Application {
                    appId: ("Netflix").to_string(),
                };
                ResponseOperator.applications.push(app);
            }
            &_ => {},
        }
    }

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

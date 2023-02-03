use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct ApplicationListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct Application {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ListApplicationsResponse {
    pub applications: Vec<Application>,
}

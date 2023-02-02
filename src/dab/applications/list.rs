use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct ApplicationListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct Application {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct ListApplicationsResponse {
    pub applications: Vec<Application>,
}

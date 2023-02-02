use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct VersionRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct Version {
    pub versions: Vec<String>,
}

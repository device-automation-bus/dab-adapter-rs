use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct VersionRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct Version {
    pub versions: Vec<String>,
}

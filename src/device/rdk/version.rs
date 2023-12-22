use crate::dab::structs::DabError;
use crate::dab::structs::Version;
use crate::dab::structs::VersionRequest;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: VersionRequest) -> Result < String, DabError > {
    let mut ResponseOperator = Version::default();
    // *** Fill in the fields of the struct Version here ***

    ResponseOperator.versions.push("2.0".to_string());

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

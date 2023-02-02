use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct KeyListRequest {}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct KeyList{
	pub keyCodes: Vec<String>,
}


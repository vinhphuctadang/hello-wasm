use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    SetNameMsg {key: String, value: String}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    GetName { name: String }
}

pub struct SetNameMsg {
    pub key: String,
    pub value: String
}

#[derive(Serialize)]
pub struct GetNameResponse {
    pub name: Option<String>
}
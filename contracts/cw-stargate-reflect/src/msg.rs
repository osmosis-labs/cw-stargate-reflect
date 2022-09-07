use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // QueryStargate relects base64 encoded proto-serialized query request
    QueryStargate {
        /// proto path for invoking query method
        path: String,
        /// `query_request` is expected to be base64 encoded due to potential utf-8 encoding issues
        query_request: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryStargateResponse {
    /// query response data in bytes, json-serialized, base64 encoded
    pub value: String,
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub kyc_provider_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AcceptTermsOfUse {},
    UpdateConfig {
        owner: Option<String>,
        kyc_provider_address: Option<String>,
    },
    AcceptOwnership {},
    RegisterAddress {
        address: String,
    },
    RegisterAddresses {
        addresses: Vec<String>,
    },
    UnregisterAddress {
        address: String,
    },
    UnregisterAddresses {
        addresses: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    IsVerified {
        address: String,
    },
    IsAccepted {
        address: String,
    },
    IsAcceptedVerified {
        address: String,
    },
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub kyc_provider_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsVerifiedResponse {
    pub address: String,
    pub is_verified: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsAcceptedResponse {
    pub address: String,
    pub is_accepted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsAcceptedVerifiedResponse {
    pub address: String,
    pub is_accepted: bool,
    pub is_verified: bool,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

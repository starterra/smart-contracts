use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128};
use crate::common::OrderBy;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub prefund_address: String,
    pub kyc_terms_vault_address: String,
    pub ido_token: String,
    pub ido_token_price: Uint128,
    pub end_date: u64,
    pub paused: bool,
    pub minimum_prefund: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    JoinIdo {},
    AcceptOwnership {},
    UpdateConfig {
        owner: Option<String>,
        prefund_address: Option<String>,
        kyc_terms_vault_address: Option<String>,
        ido_token: Option<String>,
        ido_token_price: Option<Uint128>,
        end_date: Option<u64>,
        paused: Option<bool>,
        snapshot_time: Option<u64>,
        minimum_prefund: Option<Uint128>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    FunderInfo {
        address: String,
    },
    Config {},
    State {},
    Status {
        block_time: Option<u64>,
    },
    SnapshotTime {},
    Participants {
        start_after: Option<String>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub prefund_address: String,
    pub kyc_terms_vault_address: String,
    pub ido_token: String,
    pub ido_token_price: Uint128,
    pub end_date: u64,
    pub paused: bool,
    pub snapshot_time: Option<u64>,
    pub minimum_prefund: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub number_of_participants: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticipantInfoResponse {
    pub allocation: Uint128,
    pub is_joined: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticipantResponse {
    pub is_joined: bool,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StatusResponse {
    pub is_closed: bool,
    pub is_paused: bool,
    pub snapshot_time: Option<u64>,
}

// Allocation is in ust token
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllocationInfo {
    pub address: String,
    pub allocation: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticipantsResponse {
    pub users: Vec<String>,
}


/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

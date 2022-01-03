use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub starterra_token: String,
    pub lp_staking_addresses: Vec<String>,
    pub stt_staking_addresses: Vec<String>,
    pub ido_addresses: Vec<String>,
    pub claim_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        lp_staking_addresses: Option<Vec<String>>,
        stt_staking_addresses: Option<Vec<String>>,
        ido_addresses: Option<Vec<String>>,
        claim_fee: Option<Uint128>,
    },
    EndGenesisAirdrop {},
    RegisterAirdropAccounts {
        airdrop_accounts: Vec<AirdropAccount>,
    },
    Claim {},
    UstWithdraw {
        to: String,
    },
    EmergencyWithdraw {
        amount: Uint128,
        to: String,
    },
    AcceptOwnership {},
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    UserInfo { address: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub starterra_token: String,
    pub lp_staking_addresses: Vec<String>,
    pub stt_staking_addresses: Vec<String>,
    pub ido_addresses: Vec<String>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AirdropUserInfoResponse {
    pub claimed_amount: Uint128,
    pub initial_claim_amount: Uint128,
    pub current_passed_missions: PassedMissions,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PassedMissions {
    pub is_in_lp_staking: bool,
    pub is_in_stt_staking: bool,
    pub is_in_ido: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AirdropInfo {
    pub amount: Uint128,
    pub already_claimed: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AirdropAccount {
    pub address: String,
    pub already_claimed: Uint128,
    pub amount: Uint128,
}

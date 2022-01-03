use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;
use crate::vesting::genesis::TollBridgeOptionToClaim;

/// CONTRACT: end_time > start_time
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingAccount {
    pub address: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingInfo {
    pub amount: Uint128,
    pub already_claimed: Uint128,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub starterra_token: String,
    pub genesis_time: u64,
    pub end_time: u64,
    pub name: String,
    pub paused: bool,
    pub fee_configuration: Vec<OperationFee>,
    pub treasury_address: String,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingAccountResponse {
    pub address: String,
    pub info: VestingInfo,
    pub possible_claim: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TollBridgeVestingAccountResponse {
    pub address: String,
    pub info: VestingInfo,
    pub possible_claim: Uint128,
    pub toll_bridge_available: bool,
    pub claim_options: Vec<TollBridgeOptionToClaim>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingAccountsResponse {
    pub vesting_accounts: Vec<VestingAccountResponse>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingAccountsFrozenResponse {
    pub frozen: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UserVestingResponse {
    pub is_in_vesting: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OperationFee {
    pub operation: String,
    pub fee: Uint128,
}

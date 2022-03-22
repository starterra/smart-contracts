use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::OrderBy;
use crate::vesting::common::{OperationFee, VestingAccount};
use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub starterra_token: String,
    pub genesis_time: u64,
    pub end_time: u64,
    pub name: String,
    pub toll_bridge_config: Vec<TollBridgeConfig>,
    pub paused: bool,
    pub toll_bridge_start_time: u64,
    pub toll_bridge_deadline: u64,
    pub fee_configuration: Vec<OperationFee>,
    pub treasury_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        starterra_token: Option<String>,
        genesis_time: Option<u64>,
        end_time: Option<u64>,
        name: Option<String>,
        paused: Option<bool>,
        fee_configuration: Option<Vec<OperationFee>>,
        treasury_address: Option<String>,
    },
    RegisterVestingAccounts {
        vesting_accounts: Vec<VestingAccount>,
        freeze_accounts: Option<bool>,
    },
    Claim {
        amount: Uint128,
    },
    AcceptOwnership {},
    EmergencyWithdraw {
        amount: Uint128,
        to: String,
    },
    WithdrawToBurning {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    VestingAccount {
        address: String,
        block_time: Option<u64>,
    },
    VestingAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
        block_time: Option<u64>,
    },
    VestingAccountsFrozen {},
    TollBridgeConfig {},
    UserVesting {
        address: String,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TollBridgeConfigResponse {
    pub config: Vec<TollBridgeConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TollBridgeConfig {
    pub maximum_time: Option<u64>,
    pub percentage_loss: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TollBridgeOptionToClaim {
    pub percentage_loss: u64,
    pub potential_amount: Uint128,
    pub real_amount: Uint128,
}

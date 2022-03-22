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
    pub paused: bool,
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
    UpdateVestingAccounts {
        vesting_accounts: Vec<VestingAccount>,
        freeze_accounts: Option<bool>,
    },
    SubmitToClaim {
        amount: Uint128,
    },
    Claim {
        amount: Option<Uint128>,
    },
    EmergencyWithdraw {
        amount: Uint128,
        to: String,
    },
    WithdrawToBurning {
        amount: Uint128,
    },
    AcceptOwnership {},
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
    UserVesting {
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

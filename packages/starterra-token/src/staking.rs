use cosmwasm_std::{Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::OrderBy;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub starterra_token: String,
    pub staking_token: String, // lp token of STT-UST pair contract
    pub burn_address: String,
    pub gateway_address: String,
    pub distribution_schedule: Vec<DistributionScheduleRecord>,
    pub unbond_config: Vec<UnbondConfig>,
    pub submit_to_unbond_config: SubmitToUnbondConfig,
    pub faction_name: String,
    pub fee_configuration: Vec<OperationFee>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    SubmitToUnbond {
        amount: Uint128,
    },
    Unbond {
        amount: Uint128,
    },
    InstantUnbond {
        amount: Uint128,
    },
    MoveBond {
        destination_contract: String
    },
    /// Withdraw pending rewards
    Withdraw {},
    BurningWithdraw {
        amount: Uint128,
    },
    UpdateConfig {
        owner: Option<String>,
        burn_address: Option<String>,
        gateway_address: Option<String>,
        paused: Option<bool>,
        distribution_schedule: Option<Vec<DistributionScheduleRecord>>,
        fee_configuration: Option<Vec<OperationFee>>,
        unbond_config: Option<Vec<UnbondConfig>>,
        submit_to_unbond_config: Option<SubmitToUnbondConfig>,
    },
    EmergencyWithdraw {
        amount: Uint128,
        to: String,
    },
    AcceptOwnership {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Bond {},
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {
        block_time: Option<u64>,
    },
    StakerInfo {
        staker: String,
        block_time: Option<u64>,
    },
    StakersInfo {
        start_after: Option<String>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
        block_time: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfo {
    pub reward_index: Decimal,
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
}

// We define a custom struct for config response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub starterra_token: String,
    pub staking_token: String,
    pub burn_address: String,
    pub gateway_address: String,
    pub distribution_schedule: Vec<DistributionScheduleRecord>,
    pub faction_name: String,
    pub paused: bool,
    pub max_pending_unbond_count: u64,
    pub fee_configuration: Vec<OperationFee>,
    pub unbond_config: Vec<UnbondConfig>,
    pub submit_to_unbond_config: SubmitToUnbondConfig,
}

// We define a custom struct for state response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub last_distributed: u64,
    pub total_bond_amount: Uint128,
    pub global_reward_index: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BondTimeInfo {
    pub timestamp: u64,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BondTimesInfo {
    pub bonds: Vec<BondTimeInfo>,
}

// We define a custom struct for staker info response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfoResponse {
    pub staker: String,
    pub reward_index: Decimal,
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
    pub rewards_per_fee: Vec<RewardConfig>,
    pub time_to_best_fee: Option<u64>,
    pub pending_unbond_left: Option<u64>,
    pub max_submit_to_unbond_amount: Option<Uint128>,
    pub submit_to_unbond_info: Option<BondTimesInfo>,
}

// We define a custom struct for reward config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardConfig {
    pub percent_lost: u64,
    pub amount: Uint128,
}

// We define a custom struct for stakers info response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakersInfoResponse {
    pub stakers: Vec<StakerInfoResponse>,
}

// We define a custom struct for unbond config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UnbondConfig {
    pub minimum_time: u64,
    pub percentage_loss: u64,
}

// We define a custom struct for unbond config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubmitToUnbondConfig {
    pub lock_time: u64,
}

// We define a custom struct for unbond info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UnbondInfo {
    pub submission_time: u64,
    pub amount: Uint128,
}

// We define a custom struct for unbond info response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct UnbondInfoResponse {
    pub submitted_to_unbond: Vec<UnbondInfo>,
    pub sum: Uint128,
}

// We define a custom struct for distribution schedule entities
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DistributionScheduleRecord {
    pub start_time: u64,
    pub end_time: u64,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OperationFee {
    pub operation: String,
    pub fee: Uint128,
}

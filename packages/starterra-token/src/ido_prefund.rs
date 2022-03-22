use crate::common::OrderBy;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub minimum_prefund: Uint128,
    pub fee: Uint128,
    pub anchor_market: String,
    pub anchor_ust: String,
    pub farm_in_anc: bool,
    pub min_farm_amount: Uint128,
    pub withdraw_percent_fee_nom: Uint128,
    pub withdraw_percent_fee_denom: Uint128,
    pub withdraw_max_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    PayForIdo {
        funder_reqs: Vec<FunderRequest>,
    },
    Deposit {},
    WithdrawDeposit {
        amount: Uint128,
    },
    AcceptOwnership {},
    UpdateConfig {
        owner: Option<String>,
        minimum_prefund: Option<Uint128>,
        withdrawal_active: Option<bool>,
        whitelist: Option<Vec<String>>,
        fee: Option<Uint128>,
        anchor_market: Option<String>,
        anchor_ust: Option<String>,
        farm_in_anc: Option<bool>,
        min_farm_amount: Option<Uint128>,
        withdraw_percent_fee_nom: Option<Uint128>,
        withdraw_percent_fee_denom: Option<Uint128>,
        withdraw_max_fee: Option<Uint128>,
    },
    UpdateConfigByAdmin {
        minimum_prefund: Option<Uint128>,
        withdrawal_active: Option<bool>,
    },
    DepositToAnchorByAdmin {
        amount: Uint128,
    },
    WithdrawFromAnchorByAdmin {
        amount: Uint128,
    },
    WithdrawIdoFunds {
        amount: Uint128,
        to: String,
    },
    WithdrawAnchorUST {
        amount: Option<Uint128>,
    },
    DepositAnchorUST {
        amount: Uint128,
    },
    WithdrawUST {
        amount: Option<Uint128>,
    },
    RegisterAdministrator {
        admin: String,
        is_register: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnchorUstExecuteMsg {
    RedeemStable {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    FunderInfo {
        address: String,
    },
    Funders {
        start_after: Option<String>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
    Config {},
    State {},
    IsAdmin {
        address: String,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub minimum_prefund: Uint128,
    pub withdrawal_active: bool,
    pub whitelist: Vec<String>,
    pub fee: Uint128,
    pub anchor_market: String,
    pub anchor_ust: String,
    pub farm_in_anc: bool,
    pub min_farm_amount: Uint128,
    pub withdraw_percent_fee_nom: Uint128,
    pub withdraw_percent_fee_denom: Uint128,
    pub withdraw_max_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub total_ust_balance: Uint128,
    pub ido_funds: Uint128,
    pub total_fees: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FunderInfoResponse {
    pub available_funds: Uint128,
    pub spent_funds: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FundersResponse {
    pub users: Vec<FunderResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AdminResponse {
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FunderResponse {
    pub funder: String,
    pub available_funds: Uint128,
    pub spent_funds: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FunderRequest {
    pub addr: String,
    pub amount: Uint128,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

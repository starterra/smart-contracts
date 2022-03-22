use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Deps, MessageInfo, Order, StdError, StdResult, Uint128};
use std::ops::AddAssign;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    Asc,
    Desc,
}

impl Into<Order> for OrderBy {
    fn into(self) -> Order {
        if self == OrderBy::Asc {
            Order::Ascending
        } else {
            Order::Descending
        }
    }
}

pub struct TransferBurn {
    pub transfer: Uint128,
    pub burn: Uint128,
}

impl TransferBurn {
    pub fn sum_assign(&mut self, other: Self) {
        self.transfer.add_assign(other.transfer);
        self.burn.add_assign(other.burn);
    }
}

pub fn assert_sent_native_token_balance(info: &MessageInfo, fee: Uint128) -> StdResult<()> {
    match info.funds.iter().find(|x| x.denom == "uusd".to_string()) {
        Some(coin) => {
            if fee <= coin.amount {
                Ok(())
            } else {
                let denom = coin.denom.to_string();
                let val = coin.amount;
                let together = format!("UST native token balance sent to low {} {}", denom, val);
                Err(StdError::generic_err(together))
            }
        }
        None => {
            if fee.is_zero() {
                Ok(())
            } else {
                Err(StdError::generic_err(
                    "UST native token balance sent to low",
                ))
            }
        }
    }
}

pub fn get_sent_native_token_amount(info: &MessageInfo) -> Uint128 {
    match info.funds.iter().find(|x| x.denom == "uusd".to_string()) {
        Some(coin) => coin.amount,
        None => Uint128::zero(),
    }
}

pub fn convert_human_to_raw(
    deps: Deps,
    contracts_addresses: &Vec<String>,
) -> StdResult<Vec<CanonicalAddr>> {
    contracts_addresses
        .iter()
        .map(|contract| -> StdResult<CanonicalAddr> {
            let canonical = deps.api.addr_canonicalize(&contract);
            if canonical.is_err() {
                Err::<(), StdError>(StdError::generic_err("Cannot convert address to canonical"))?;
            }
            canonical
        })
        .collect::<StdResult<Vec<CanonicalAddr>>>()
}

pub fn convert_raw_to_human(deps: Deps, addresses: &Vec<CanonicalAddr>) -> StdResult<Vec<String>> {
    addresses
        .iter()
        .map(|addr| -> StdResult<String> {
            let human_addr = deps.api.addr_humanize(&addr);
            if human_addr.is_err() {
                Err::<(), StdError>(StdError::generic_err("Cannot convert address to human"))?;
            }
            Ok(human_addr?.into_string())
        })
        .collect::<StdResult<Vec<String>>>()
}

use starterra_token::staking::StakerInfoResponse;

use crate::querier::load_user_staking_status;
use crate::state::{Config, read_config};
use crate::errors::ContractError;
use cosmwasm_std::{Deps, Uint128, StdResult, StdError};

pub fn fetch_staking_statuses(
    deps: Deps,
    account_addr: &String,
) -> StdResult<Vec<(String, StakerInfoResponse)>> {
    let config: Config = read_config(deps.storage)?;
    let raw_addr = deps.api.addr_canonicalize(account_addr)?;

    config.staking_contracts.iter().map(|contract_addr| {
        load_user_staking_status(deps, &deps.api.addr_humanize(contract_addr)?.into_string(), &raw_addr)
    }).collect::<StdResult<Vec<(String, StakerInfoResponse)>>>()
}

pub fn is_user_staking(
    staking_statuses: &Vec<(String, StakerInfoResponse)>,
) -> bool {
    for status in staking_statuses {
        if status.1.bond_amount != Uint128::zero() {
            return true;
        }
    }

    return false;
}

pub fn get_staking_amount(
    staking_statuses: &Vec<(String, StakerInfoResponse)>,
) -> Result<(Option<String>, Uint128), ContractError> {
    let mut staking_amount = Uint128::zero();
    let mut contract_addr: Option<String> = None;
    for status in staking_statuses {
        if !staking_amount.is_zero() && !status.1.bond_amount.is_zero() {
            return Err(ContractError::CannotStakeInMoreThanOneContract {});
        }

        if status.1.bond_amount != Uint128::zero() {
            staking_amount = status.1.bond_amount;
            contract_addr = Some(status.clone().0);
        }
    }

    return Ok((contract_addr, staking_amount));
}

pub fn assert_staking_contracts_len(staking_contracts: &Vec<String>) -> StdResult<()>
{
    if staking_contracts.len() > 5 {
        return Err(StdError::generic_err("Maximum number of staking contracts is 5"));
    }
    Ok(())
}

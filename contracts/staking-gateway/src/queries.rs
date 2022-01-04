
use starterra_token::staking_gateway::{AddressesResponse, CanStakeResponse, CanStakeStatus, ConfigResponse, BondAmountResponse};

use crate::state::{Config, read_config};
use crate::errors::ContractError;
use crate::tools::{is_user_staking, get_staking_amount, fetch_staking_statuses};
use cosmwasm_std::{Deps, Uint128};
use starterra_token::common::convert_raw_to_human;

pub fn query_config(
    deps: Deps,
) -> Result<ConfigResponse, ContractError> {
    let state = read_config(deps.storage)?;
    let resp = ConfigResponse {
        owner: deps.api.addr_humanize(&state.owner)?.into_string(),
        staking_contracts: convert_raw_to_human(deps, &state.staking_contracts)?,
    };

    Ok(resp)
}

pub fn query_can_stake(
    deps: Deps,
    user: String,
) -> Result<CanStakeResponse, ContractError> {
    let staking_statuses = fetch_staking_statuses(deps, &user)?;
    let mut statuses = Vec::with_capacity(staking_statuses.len());

    if is_user_staking(&staking_statuses) {
        for s in staking_statuses {
            statuses.push(CanStakeStatus {
                staking_contract: s.0,
                can_stake: s.1.bond_amount != Uint128::zero(), // because can stake only in the one which is already staking
            });
        }

        if statuses.iter().filter(|can_stake| can_stake.can_stake == true).count() != 1 {
            return Err(ContractError::CannotStakeInMoreThanOneContract {});
        }
    } else {
        for s in staking_statuses {
            statuses.push(CanStakeStatus {
                staking_contract: s.0,
                can_stake: true,
            });
        }
    }

    return Ok(CanStakeResponse {
        statuses,
    });
}


pub fn query_bond_amount(
    deps: Deps,
    user: String,
) -> Result<BondAmountResponse, ContractError> {
    let staking_statuses = fetch_staking_statuses(deps, &user)?;
    let staking_info = get_staking_amount(&staking_statuses)?;

    return Ok(BondAmountResponse {
        user,
        contract: staking_info.0,
        bond_amount: staking_info.1,
    });
}

pub fn query_addresses(
    deps: Deps,
) -> Result<AddressesResponse, ContractError> {
    let config: Config = read_config(deps.storage)?;

    let addresses: Vec<String> = config.staking_contracts
        .iter()
        .map(|contract_addr| {
            let addr = deps.api.addr_humanize(contract_addr);
            addr.unwrap().into_string()
        }).collect::<Vec<String>>();

    return Ok(AddressesResponse {
        addresses
    });
}

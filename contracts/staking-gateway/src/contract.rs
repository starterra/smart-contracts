#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use starterra_token::staking_gateway::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::errors::ContractError;
use crate::execute::{accept_ownership, update_config};
use crate::queries::{query_addresses, query_bond_amount, query_can_stake, query_config};
use crate::state::{store_config, Config};
use crate::tools::assert_staking_contracts_len;
use starterra_token::common::convert_human_to_raw;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    assert_staking_contracts_len(&msg.staking_contracts)?;

    let config = Config {
        owner: deps.api.addr_canonicalize(&msg.owner)?,
        staking_contracts: convert_human_to_raw(deps.as_ref(), &msg.staking_contracts)?,
    };

    store_config(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            staking_contracts,
        } => update_config(deps, info, owner, staking_contracts),
        ExecuteMsg::AcceptOwnership {} => accept_ownership(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Config {} => Ok(to_binary(&query_config(deps)?)?),
        QueryMsg::CanUserStake { user } => Ok(to_binary(&query_can_stake(deps, user)?)?),
        QueryMsg::BondAmount { user } => Ok(to_binary(&query_bond_amount(deps, user)?)?),
        QueryMsg::Addresses {} => Ok(to_binary(&query_addresses(deps)?)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

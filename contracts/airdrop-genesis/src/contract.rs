use cosmwasm_std::{Binary, Env, StdResult, to_binary, MessageInfo, DepsMut, Deps, Response};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use starterra_token::airdrop_genesis::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::execute::{claim, emergency_withdraw, end_airdrop_genesis, register_airdrop_accounts, update_config, ust_withdraw, accept_ownership};
use crate::queries::{query_config, query_user_info};
use crate::state::{Config, store_config};
use crate::tools::{assert_owner_privilege, convert_human_to_raw};
use crate::errors::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let lp_staking_addresses = convert_human_to_raw(deps.as_ref(), &msg.lp_staking_addresses)?;
    let stt_staking_addresses = convert_human_to_raw(deps.as_ref(), &msg.stt_staking_addresses)?;
    let ido_addresses = convert_human_to_raw(deps.as_ref(), &msg.ido_addresses)?;
    store_config(
        deps.storage,
        &Config {
            owner: deps.api.addr_canonicalize(&msg.owner)?,
            starterra_token: deps.api.addr_canonicalize(&msg.starterra_token)?,
            lp_staking_addresses,
            stt_staking_addresses,
            ido_addresses,
            claim_fee: msg.claim_fee,
        },
    )?;


    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg.clone() {
        ExecuteMsg::Claim {} => claim(deps, info),
        ExecuteMsg::AcceptOwnership {} => {
            accept_ownership(deps, info)
        },
        _ => {
            assert_owner_privilege(deps.as_ref(), info.clone())?;
            match msg {
                ExecuteMsg::UpdateConfig {
                    owner,
                    lp_staking_addresses,
                    stt_staking_addresses,
                    ido_addresses,
                    claim_fee
                } => update_config(deps, env, owner, lp_staking_addresses, stt_staking_addresses, ido_addresses, claim_fee),
                ExecuteMsg::EndGenesisAirdrop {} => end_airdrop_genesis(deps, env),
                ExecuteMsg::RegisterAirdropAccounts { airdrop_accounts } => {
                    register_airdrop_accounts(deps, &airdrop_accounts)
                }
                ExecuteMsg::UstWithdraw { to } => {
                    ust_withdraw(deps, env, to)
                }
                ExecuteMsg::EmergencyWithdraw { amount, to } => {
                    emergency_withdraw(deps, env, amount, to)
                }
                _ => panic!("DO NOT ENTER HERE"),
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Config {} => Ok(to_binary(&query_config(deps)?)?),
        QueryMsg::UserInfo { address } => {
            Ok(to_binary(&query_user_info(deps, address)?)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

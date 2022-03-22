#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::errors::ContractError;
use crate::execute::{accept_ownership, join_ido, update_config};
use crate::queries::{
    query_config, query_ido_state, query_ido_status, query_participant, query_participants,
    query_snapshot_time,
};
use crate::state::{store_config, store_state, Config, State};
use crate::tools::assert_owner_privilege;
use starterra_token::ido::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if msg.end_date <= env.block.time.seconds() {
        return Err(ContractError::EndDateInThePast {});
    }

    store_config(
        deps.storage,
        &Config {
            owner: deps.api.addr_canonicalize(&msg.owner)?,
            pending_owner: None,
            prefund_address: deps.api.addr_canonicalize(&msg.prefund_address)?,
            kyc_terms_vault_address: deps.api.addr_canonicalize(&msg.kyc_terms_vault_address)?,
            ido_token: deps.api.addr_canonicalize(&msg.ido_token)?,
            ido_token_price: msg.ido_token_price,
            end_date: msg.end_date,
            paused: msg.paused,
            snapshot_time: None,
            minimum_prefund: msg.minimum_prefund,
        },
    )?;

    store_state(
        deps.storage,
        &State {
            number_of_participants: 0,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg.clone() {
        ExecuteMsg::JoinIdo {} => join_ido(deps, info.clone(), env),
        ExecuteMsg::AcceptOwnership {} => accept_ownership(deps, info),
        _ => {
            assert_owner_privilege(deps.as_ref(), info.clone())?;
            match msg {
                ExecuteMsg::UpdateConfig {
                    owner,
                    prefund_address,
                    kyc_terms_vault_address: kyc_vault_address,
                    ido_token,
                    ido_token_price,
                    end_date,
                    paused,
                    snapshot_time,
                    minimum_prefund,
                } => update_config(
                    deps,
                    info,
                    env,
                    owner,
                    prefund_address,
                    kyc_vault_address,
                    ido_token,
                    ido_token_price,
                    end_date,
                    paused,
                    snapshot_time,
                    minimum_prefund,
                ),
                _ => panic!("DO NOT ENTER HERE"),
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_ido_state(deps)?),
        QueryMsg::Status { block_time } => to_binary(&query_ido_status(deps, env, block_time)?),
        QueryMsg::FunderInfo { address } => to_binary(&query_participant(deps, address)?),
        QueryMsg::SnapshotTime {} => to_binary(&query_snapshot_time(deps)?),
        QueryMsg::Participants {
            start_after,
            limit,
            order_by,
        } => to_binary(&query_participants(deps, start_after, limit, order_by)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

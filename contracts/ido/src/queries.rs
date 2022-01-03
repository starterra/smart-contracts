use cosmwasm_std::{StdResult, Deps, Env};
use crate::state::{read_config, read_state, read_participant, read_participants};
use starterra_token::ido::{ConfigResponse, StateResponse, StatusResponse, ParticipantResponse, ParticipantsResponse};
use std::borrow::Borrow;
use starterra_token::common::{OrderBy, convert_raw_to_human};

pub fn query_config(
    deps: Deps,
) -> StdResult<ConfigResponse> {
    let state = read_config(deps.storage)?;
    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&state.owner)?.into_string(),
        prefund_address: deps.api.addr_humanize(&state.prefund_address)?.into_string(),
        kyc_terms_vault_address: deps.api.addr_humanize(&state.kyc_terms_vault_address)?.into_string(),
        ido_token: deps.api.addr_humanize(&state.ido_token)?.into_string(),
        ido_token_price: state.ido_token_price,
        end_date: state.end_date,
        paused: state.paused,
        snapshot_time: state.snapshot_time,
        minimum_prefund: state.minimum_prefund,
    })
}

pub fn query_ido_state(
    deps: Deps,
) -> StdResult<StateResponse> {
    let state = read_state(deps.storage)?;
    Ok(StateResponse {
        number_of_participants: state.number_of_participants,
    })
}

pub fn query_participant(
    deps: Deps,
    address: String,
) -> StdResult<ParticipantResponse> {
    let user_raw = deps.api.addr_canonicalize(&address)?;
    let participant = read_participant(deps.storage.borrow(), &user_raw);
    match participant {
        Ok(res) => {
            Ok(res)
        }
        _ => {
            Ok(ParticipantResponse {
                is_joined: false,
            })
        }
    }
}

pub fn query_ido_status(
    deps: Deps,
    env: Env,
    block_time: Option<u64>,
) -> StdResult<StatusResponse> {
    let state = read_config(deps.storage)?;
    let is_closed = state.end_date < block_time.unwrap_or(env.block.time.seconds());
    Ok(StatusResponse {
        is_closed,
        is_paused: state.paused,
        snapshot_time: state.snapshot_time,
    })
}

pub fn query_snapshot_time(
    deps: Deps,
) -> StdResult<Option<u64>> {
    let state = read_config(deps.storage)?;
    match state.snapshot_time {
        Some(s) => Ok(Some(s)),
        None => Ok(None)
    }
}

pub fn query_participants(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<ParticipantsResponse> {
    let users = if let Some(start_after) = start_after {
        read_participants(
            deps.storage,
            Some(deps.api.addr_canonicalize(&start_after)?),
            limit,
            order_by,
        )?
    } else {
        read_participants(deps.storage, None, limit, order_by)?
    };

    return Ok(ParticipantsResponse { users: convert_raw_to_human(deps, &users)? });
}

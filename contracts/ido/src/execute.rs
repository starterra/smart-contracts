use cosmwasm_std::{Uint128, DepsMut, MessageInfo, Env, Response};
use crate::state::{read_config, store_config, read_state, store_state, Config, read_participant, store_participant};
use crate::querier::{load_user_prefund_balance, check_user_kyc_terms_verified};
use crate::errors::ContractError;

pub fn join_ido(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
) -> Result<Response, ContractError> {
    let sender = deps.api.addr_canonicalize(&info.sender.clone().into_string())?;
    let mut ido_participant =
        read_participant(
            deps.storage,
            &sender,
        )?;

    if ido_participant.is_joined {
        return Err(ContractError::AlreadyJoined {});
    }

    let config = read_config(deps.storage)?;
    if config.paused {
        return Err(ContractError::IdoPaused {});
    }
    if env.block.time.seconds() > config.end_date {
        return Err(ContractError::IdoClosed {});
    }

    let funder_info = load_user_prefund_balance(
        &deps.querier,
        deps.api.addr_humanize(&config.prefund_address)?.into_string(),
        info.sender.clone().into_string(),
    )?;

    // check if deposited enough
    if funder_info.available_funds < config.minimum_prefund {
        return Err(ContractError::NotEnoughDeposit {});
    }

    //check if kyc confirmed
    let kyc_terms_info = check_user_kyc_terms_verified(&deps.querier, deps.api.addr_humanize(&config.kyc_terms_vault_address)?.into_string(), info.sender.clone().into_string())?;
    if !kyc_terms_info.is_verified {
        return Err(ContractError::KycFailed {});
    }
    if !kyc_terms_info.is_accepted {
        return Err(ContractError::TouFailed {});
    }

    let mut state = read_state(deps.storage)?;
    state.number_of_participants += 1;
    store_state(deps.storage, &state)?;

    ido_participant.is_joined = true;
    store_participant(deps.storage, &sender, &ido_participant)?;

    Ok(Response::new()
        .add_attribute("action", "join_ido")
        .add_attribute("address", info.sender.into_string()))
}

pub fn accept_ownership(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;
    match config.pending_owner {
        None => {
            return Err(ContractError::PendingOwnerMissing {});
        }
        Some(pending_owner) => {
            if deps.api.addr_canonicalize(info.sender.as_str())? != pending_owner {
                return Err(ContractError::Unauthorized {});
            }

            config.owner = pending_owner;
            config.pending_owner = None;
            store_config(deps.storage, &config)?;
        }
    }

    Ok(Response::new()
        .add_attribute("action", "accept_ownership")
        .add_attribute("owner", info.sender)
    )
}

pub fn update_config(
    deps: DepsMut,
    _info: MessageInfo,
    env: Env,
    owner: Option<String>,
    prefund_address: Option<String>,
    kyc_vault_address: Option<String>,
    ido_token: Option<String>,
    ido_token_price: Option<Uint128>,
    end_date: Option<u64>,
    paused: Option<bool>,
    snapshot_time: Option<u64>,
    minimum_prefund: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config = read_config(deps.storage)?;

    if let Some(owner) = owner {
        config.pending_owner = Some(deps.api.addr_canonicalize(&owner)?);
    }

    if let Some(prefund_address) = prefund_address {
        config.prefund_address = deps.api.addr_canonicalize(&prefund_address)?;
    }

    if let Some(kyc_vault_address) = kyc_vault_address {
        config.kyc_terms_vault_address = deps.api.addr_canonicalize(&kyc_vault_address)?;
    }

    if let Some(ido_token) = ido_token {
        config.ido_token = deps.api.addr_canonicalize(&ido_token)?;
    }

    if let Some(ido_token_price) = ido_token_price {
        config.ido_token_price = ido_token_price;
    }

    if let Some(end_date) = end_date {
        if end_date <= env.block.time.seconds() {
            return Err(ContractError::EndDateInThePast {});
        }
        config.end_date = end_date;
    }

    if let Some(paused) = paused {
        config.paused = paused;
    }

    if let Some(minimum_prefund) = minimum_prefund {
        config.minimum_prefund = minimum_prefund;
    }

    if let Some(snapshot_time) = snapshot_time {
        if env.block.time.seconds() > snapshot_time {
            return Err(ContractError::SnapshotTimeFromPast {});
        }
        config.snapshot_time = Some(snapshot_time);
    }

    store_config(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config"))
}

use cosmwasm_std::{ DepsMut, MessageInfo, Response};

use crate::state::{Config, store_config, read_config, store_pending_owner, remove_pending_owner, read_pending_owner};
use crate::errors::ContractError;
use starterra_token::common::convert_human_to_raw;
use crate::tools::assert_staking_contracts_len;

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<String>,
    staking_contracts: Option<Vec<String>>,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;
    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        store_pending_owner(deps.storage, &deps.api.addr_canonicalize(&owner)?)?;
    }

    if let Some(staking_contracts) = staking_contracts {
        assert_staking_contracts_len(&staking_contracts)?;
        config.staking_contracts = convert_human_to_raw(deps.as_ref(), &staking_contracts)?;
    }

    store_config(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn accept_ownership(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    match read_pending_owner(deps.storage) {
        None => {
            return Err(ContractError::PendingOwnerMissing {});
        }
        Some(pending_owner) => {
            let mut config: Config = read_config(deps.storage)?;
            if deps.api.addr_canonicalize(&info.sender.to_string())? != pending_owner {
                return Err(ContractError::Unauthorized {});
            }

            config.owner = pending_owner;
            store_config(deps.storage, &config)?;
            remove_pending_owner(deps.storage);
        }
    }

    Ok(Response::new()
        .add_attribute("action", "accept_ownership")
        .add_attribute("owner", info.sender))
}

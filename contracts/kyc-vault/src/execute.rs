use cosmwasm_std:: {DepsMut, Response, MessageInfo};
use crate::state::{store_kyc_address, read_tou_address, store_tou_address, read_config, store_config, read_pending_owner, Config, remove_pending_owner, store_pending_owner};
use starterra_token::common::convert_human_to_raw;
use crate::errors::ContractError;

pub fn register_kyc_account(
    deps: DepsMut,
    address: &String,
    is_registering: bool,
) -> Result<Response, ContractError> {
    let kyc_address = deps.api.addr_canonicalize(address)?;
    store_kyc_address(
        deps.storage,
        &kyc_address,
        is_registering,
    )?;

    let log_action_msg = if is_registering { "register_kyc_address" } else { "unregister_kyc_address" };
    let log_msg = if is_registering { "registered_kyc_address" } else { "unregistered_kyc_address" };

    Ok(Response::new()
        .add_attribute("action", log_action_msg)
        .add_attribute(log_msg, address)
    )
}

pub fn register_kyc_accounts(
    deps: DepsMut,
    addresses: &Vec<String>,
    is_registering: bool,
) -> Result<Response, ContractError> {
    let kyc_addresses = convert_human_to_raw(deps.as_ref(), addresses)?;
    for kyc_address in kyc_addresses {
        store_kyc_address(
            deps.storage,
            &kyc_address,
            is_registering,
        )?;
    }

    let log_action_msg = if is_registering { "register_kyc_addresses" } else { "unregister_kyc_addresses" };
    let log_msg = if is_registering { "registered_kyc_addresses" } else { "unregistered_kyc_addresses" };
    let log_addresses: Vec<String> = addresses.into_iter().map(|x| x.to_string()).collect();

    Ok(Response::new()
        .add_attribute("action", log_action_msg)
        .add_attribute(log_msg, log_addresses.join(","))
    )
}

pub fn accept_terms_of_use(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let user_address = deps.api.addr_canonicalize(info.sender.as_str())?;
    let account_accepted = read_tou_address(deps.storage, &user_address)?;

    if account_accepted {
        return Err(ContractError::TouAlreadyAccepted {});
    }

    store_tou_address(
        deps.storage,
        &user_address,
        true,
    )?;

    Ok(Response::new()
        .add_attribute("action", "accept_terms_of_use")
        .add_attribute("address", info.sender.as_str())
    )
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<String>,
    kyc_provider_address: Option<String>,
) -> Result<Response, ContractError> {
    if read_config(deps.storage)?.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }
    let mut config = read_config(deps.storage)?;

    if let Some(owner) = owner {
        store_pending_owner(deps.storage, &deps.api.addr_canonicalize(&owner)?)?;
    }

    if let Some(kyc_provider_address) = kyc_provider_address {
        config.kyc_provider_address = deps.api.addr_canonicalize(&kyc_provider_address)?;
    }

    store_config(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
    )
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

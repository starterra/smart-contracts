#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CanonicalAddr, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use starterra_token::vesting_gateway::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, VestingAddressesResponse,
    VestingByUserResponse,
};

use crate::errors::ContractError;
use crate::querier::query_is_address_on_vesting;
use crate::state::{
    read_config, read_pending_owner, read_vesting_addresses, remove_pending_owner, store_config,
    store_pending_owner, store_vesting_addresses, Config,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    store_config(
        deps.storage,
        &Config {
            owner: deps.api.addr_canonicalize(&msg.owner)?,
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
        ExecuteMsg::AcceptOwnership {} => accept_ownership(deps, info),
        _ => {
            assert_owner_privilege(deps.as_ref(), info.clone())?;
            match msg {
                ExecuteMsg::UpdateConfig { owner } => update_config(deps, env, owner),
                ExecuteMsg::UpdateVestingAddresses { vesting_addresses } => {
                    update_vesting_addresses(deps, env, vesting_addresses)
                }
                ExecuteMsg::AddVestingAddress { vesting_address } => {
                    add_vesting_address(deps, env, vesting_address)
                }
                ExecuteMsg::RemoveVestingAddress { vesting_address } => {
                    remove_vesting_address(deps, env, vesting_address)
                }
                _ => panic!("DO NOT ENTER HERE"),
            }
        }
    }
}

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    owner: Option<String>,
) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;

    if let Some(owner) = owner {
        store_pending_owner(deps.storage, &deps.api.addr_canonicalize(&owner)?)?;
    }

    store_config(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn accept_ownership(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
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

pub fn update_vesting_addresses(
    deps: DepsMut,
    _env: Env,
    vesting_addresses: Vec<String>,
) -> Result<Response, ContractError> {
    if vesting_addresses.len() > 6 {
        return Err(ContractError::CannotHaveMoreVestingAddresses { max: 6 });
    }

    let addresses_with_errors: (Vec<_>, Vec<_>) = vesting_addresses
        .into_iter()
        .map(|address| deps.api.addr_canonicalize(&address))
        .partition(Result::is_ok);

    let addresses: Vec<_> = addresses_with_errors
        .0
        .into_iter()
        .map(Result::unwrap)
        .collect();

    store_vesting_addresses(deps.storage, &addresses)?;

    Ok(Response::new().add_attribute("action", "update_vesting_addresses"))
}

pub fn add_vesting_address(
    deps: DepsMut,
    _env: Env,
    vesting_address: String,
) -> Result<Response, ContractError> {
    let address_raw = deps.api.addr_canonicalize(&vesting_address)?;
    let mut addresses = read_vesting_addresses(deps.storage)?;
    if addresses.len() > 5 {
        return Err(ContractError::CannotAddMoreVestingAddresses { max: 6 });
    }
    for address in addresses.clone().into_iter() {
        if address == address_raw {
            return Err(ContractError::AddressAlreadyRegistered {});
        }
    }

    addresses.push(address_raw);

    store_vesting_addresses(deps.storage, &addresses)?;

    Ok(Response::new()
        .add_attribute("action", "add_vesting_addresses")
        .add_attribute("new_vesting_address", vesting_address))
}

pub fn remove_vesting_address(
    deps: DepsMut,
    _env: Env,
    vesting_address: String,
) -> Result<Response, ContractError> {
    let address_raw = deps.api.addr_canonicalize(&vesting_address)?;
    let mut addresses = read_vesting_addresses(deps.storage)?;

    let index = addresses.iter().position(|x| *x == address_raw);
    if index == None {
        return Err(ContractError::AddressNotRegistered {});
    }
    addresses.remove(index.unwrap());

    store_vesting_addresses(deps.storage, &addresses)?;

    Ok(Response::new()
        .add_attribute("action", "removed_vesting_addresses")
        .add_attribute("removed_vesting_address", vesting_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::VestingAddresses {} => to_binary(&query_vesting_addresses(deps)?),
        QueryMsg::FindVestingByUser { user_address } => {
            to_binary(&query_vesting_by_user(deps, user_address)?)
        }
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = read_config(deps.storage)?;
    let resp = ConfigResponse {
        owner: deps.api.addr_humanize(&state.owner)?.into_string(),
    };

    Ok(resp)
}

pub fn query_vesting_addresses(deps: Deps) -> StdResult<VestingAddressesResponse> {
    let addresses = read_vesting_addresses(deps.storage)?;

    let addresses_with_errors: (Vec<_>, Vec<_>) = addresses
        .into_iter()
        .map(|address| deps.api.addr_humanize(&address))
        .partition(Result::is_ok);

    let addresses: Vec<_> = addresses_with_errors
        .0
        .into_iter()
        .map(Result::unwrap)
        .map(|value| value.into_string())
        .collect();

    Ok(VestingAddressesResponse {
        vesting_addresses: addresses,
    })
}

pub fn query_vesting_by_user(deps: Deps, address: String) -> StdResult<VestingByUserResponse> {
    let vesting_addresses = read_vesting_addresses(deps.storage)?;

    let mut res_address: Option<CanonicalAddr> = None;
    for vesting_address in vesting_addresses.into_iter() {
        let is_found = query_is_address_on_vesting(
            deps,
            deps.api.addr_humanize(&vesting_address)?.into_string(),
            deps.api.addr_canonicalize(&address)?,
        );
        match is_found {
            Ok(v) => {
                if v.is_in_vesting == true {
                    res_address = Some(vesting_address.clone());
                    break;
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    if res_address != None {
        return Ok(VestingByUserResponse {
            vesting_address: Some(deps.api.addr_humanize(&res_address.unwrap())?.into_string()),
        });
    }
    Ok(VestingByUserResponse {
        vesting_address: None,
    })
}

pub fn assert_owner_privilege(deps: Deps, info: MessageInfo) -> Result<(), ContractError> {
    if crate::state::read_config(deps.storage)?.owner
        != deps.api.addr_canonicalize(info.sender.as_str())?
    {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

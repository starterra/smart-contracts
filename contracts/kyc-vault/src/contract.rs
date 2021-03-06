#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std:: {Env, Response, StdResult, Binary, to_binary, DepsMut, MessageInfo, Deps};

use starterra_token::kyc_vault::{MigrateMsg, QueryMsg, InstantiateMsg, ExecuteMsg};
use crate::state::{store_config, Config};
use crate::execute::{register_kyc_account, register_kyc_accounts, accept_terms_of_use, update_config, accept_ownership};
use crate::tools::{assert_kyc_provider_privilege};
use crate::queries::{query_config, query_verified, query_accepted, query_accepted_verified};
use crate::errors::ContractError;

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
            kyc_provider_address: deps.api.addr_canonicalize(&msg.kyc_provider_address)?,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg.clone() {
        ExecuteMsg::AcceptTermsOfUse {} => accept_terms_of_use(deps, info.clone()),
        ExecuteMsg::UpdateConfig {
            owner,
            kyc_provider_address,
        } => update_config(deps, info, owner, kyc_provider_address),
        ExecuteMsg::AcceptOwnership {} => accept_ownership(deps, info),
        _ => {
            assert_kyc_provider_privilege(deps.as_ref(), info)?;
            match msg {
                ExecuteMsg::RegisterAddress { address } => {
                    register_kyc_account(deps, &address, true)
                }
                ExecuteMsg::RegisterAddresses { addresses } => {
                    register_kyc_accounts(deps, &addresses, true)
                }
                ExecuteMsg::UnregisterAddress { address } => {
                    register_kyc_account(deps, &address, false)
                }
                ExecuteMsg::UnregisterAddresses { addresses } => {
                    register_kyc_accounts(deps, &addresses, false)
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
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::IsVerified { address } => {
            to_binary(&query_verified(deps, address)?)
        }
        QueryMsg::IsAccepted { address } => {
            to_binary(&query_accepted(deps, address)?)
        }
        QueryMsg::IsAcceptedVerified { address } => {
            to_binary(&query_accepted_verified(deps, address)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}


use cosmwasm_std::{StdResult, Deps};
use crate::state::{read_config, read_kyc_address, read_tou_address};
use starterra_token::kyc_vault::{ConfigResponse, IsVerifiedResponse, IsAcceptedVerifiedResponse, IsAcceptedResponse};

pub fn query_config(
    deps: Deps,
) -> StdResult<ConfigResponse> {
    let state = read_config(deps.storage)?;
    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&state.owner)?.into_string(),
        kyc_provider_address: deps.api.addr_humanize(&state.kyc_provider_address)?.into_string(),
    })
}

pub fn query_verified(
    deps: Deps,
    address: String,
) -> StdResult<IsVerifiedResponse> {
    let user_raw = deps.api.addr_canonicalize(&address)?;
    let is_verified = read_kyc_address(deps.storage, &user_raw).unwrap_or(false);
    Ok(IsVerifiedResponse {
        address,
        is_verified,
    })
}


pub fn query_accepted(
    deps: Deps,
    address: String,
) -> StdResult<IsAcceptedResponse> {
    let user_raw = deps.api.addr_canonicalize(&address)?;
    let is_accepted = read_tou_address(deps.storage, &user_raw).unwrap_or(false);
    Ok(IsAcceptedResponse {
        address,
        is_accepted,
    })
}

pub fn query_accepted_verified(
    deps: Deps,
    address: String,
) -> StdResult<IsAcceptedVerifiedResponse> {
    let user_raw = deps.api.addr_canonicalize(&address)?;
    let is_accepted = read_tou_address(deps.storage, &user_raw).unwrap_or_else(|_| false);
    let is_verified = read_kyc_address(deps.storage, &user_raw).unwrap_or_else(|_| false);

    Ok(IsAcceptedVerifiedResponse {
        address,
        is_accepted,
        is_verified,
    })
}

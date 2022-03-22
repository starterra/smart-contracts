use crate::errors::ContractError;
use crate::state::{read_airdrop_info, read_config, Config};
use crate::tools::{convert_raw_to_human, user_ido_check, user_stake_check};
use cosmwasm_std::Deps;
use starterra_token::airdrop_genesis::{AirdropUserInfoResponse, ConfigResponse, PassedMissions};

pub fn query_config(deps: Deps) -> Result<ConfigResponse, ContractError> {
    let state = read_config(deps.storage)?;
    let resp = ConfigResponse {
        owner: deps.api.addr_humanize(&state.owner)?.into_string(),
        starterra_token: deps
            .api
            .addr_humanize(&state.starterra_token)?
            .into_string(),
        lp_staking_addresses: convert_raw_to_human(deps, &state.lp_staking_addresses)?,
        stt_staking_addresses: convert_raw_to_human(deps, &state.stt_staking_addresses)?,
        ido_addresses: convert_raw_to_human(deps, &state.ido_addresses)?,
    };

    Ok(resp)
}

pub fn query_user_info(
    deps: Deps,
    address: String,
) -> Result<AirdropUserInfoResponse, ContractError> {
    let config: Config = read_config(deps.storage)?;
    let user_raw = deps.api.addr_canonicalize(&address)?;
    let airdrop_info = read_airdrop_info(deps.storage, &user_raw)?;
    Ok(AirdropUserInfoResponse {
        claimed_amount: airdrop_info.already_claimed,
        initial_claim_amount: airdrop_info.amount,
        current_passed_missions: PassedMissions {
            is_in_lp_staking: user_stake_check(deps, &user_raw, &config.lp_staking_addresses),
            is_in_stt_staking: user_stake_check(deps, &user_raw, &config.stt_staking_addresses),
            is_in_ido: user_ido_check(deps, &user_raw, &config.ido_addresses),
        },
    })
}

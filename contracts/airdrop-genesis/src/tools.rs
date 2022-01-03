use cosmwasm_std::{Env, StdResult, Uint128, CanonicalAddr, Coin, Deps, MessageInfo};
use crate::state::{read_config, Config};
use crate::querier::{load_balance, check_if_user_stakes, check_if_user_participated_in_ido};
use crate::errors::ContractError;


pub fn convert_human_to_raw(
    deps: Deps,
    staking_contracts: &Vec<String>,
) -> Result<Vec<CanonicalAddr>, ContractError> {
    staking_contracts.iter()
        .map(|contract| -> Result<CanonicalAddr, ContractError> {
            let canonical = deps.api.addr_canonicalize(&contract);
            if canonical.is_err() {
                Err(ContractError::CannotConvertAddressToCanonical {})?;
            }
            Ok(canonical.unwrap())
        })
        .collect::<Result<Vec<CanonicalAddr>, ContractError>>()
}

pub fn convert_raw_to_human(
    deps: Deps,
    staking_contracts: &Vec<CanonicalAddr>,
) -> Result<Vec<String>, ContractError> {
    staking_contracts.iter()
        .map(|contract| -> Result<String, ContractError> {
            let human_addr = deps.api.addr_humanize(&contract);
            if human_addr.is_err() {
                Err(ContractError::CannotConvertAddressToHuman {})?;
            }
            Ok(human_addr.unwrap().into_string())
        })
        .collect::<Result<Vec<String>, ContractError>>()
}

pub fn fetch_user_possible_claim(
    deps: Deps,
    account_addr: &CanonicalAddr,
    base_claim: Uint128,
) -> StdResult<Uint128> {
    let config: Config = read_config(deps.storage)?;
    let mut nominator = 1u128;
    let denominator = 4u128;

    let is_lp_staking = user_stake_check(deps, account_addr, &config.lp_staking_addresses);
    if is_lp_staking {
        nominator += 1;
    }

    let is_stt_staking = user_stake_check(deps, account_addr, &config.stt_staking_addresses);
    if is_stt_staking {
        nominator += 1;
    }

    let participated_in_ido = user_ido_check(deps, account_addr, &config.ido_addresses);
    if participated_in_ido {
        nominator += 1;
    }

    return Ok(base_claim.multiply_ratio(nominator, denominator));
}

pub fn user_stake_check(
    deps: Deps,
    account_addr: &CanonicalAddr,
    staking_addresses: &Vec<CanonicalAddr>,
) -> bool {
    let staking_res: Result<Vec<bool>, ContractError> = staking_addresses.iter().map(|contract_addr| {
        check_if_user_stakes(deps, &deps.api.addr_humanize(contract_addr)?.into_string(), account_addr)
    }).collect::<Result<Vec<bool>, ContractError>>();

    staking_res.unwrap().iter().any(|x| *x)
}

pub fn user_ido_check(
    deps: Deps,
    account_addr: &CanonicalAddr,
    ido_addresses: &Vec<CanonicalAddr>,
) -> bool {
    let ido_res: Result<Vec<bool>, ContractError> = ido_addresses.iter().map(|contract_addr| {
        check_if_user_participated_in_ido(deps, &deps.api.addr_humanize(contract_addr)?.into_string(), account_addr)
    }).collect::<Result<Vec<bool>, ContractError>>();

    ido_res.unwrap().iter().any(|x| *x)
}

pub fn assert_owner_privilege(
    deps: Deps,
    info: MessageInfo,
) -> Result<(), ContractError> {
    if read_config(deps.storage)?.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn get_ust_withdraw_coin(
    deps: Deps,
    env: Env,
) -> Result<Coin, ContractError> {
    let denom = String::from("uusd");
    let balance = load_balance(deps, &env.contract.address.into_string(), denom.clone())?;
    Ok(Coin { denom, amount: balance })
}

pub fn assert_sent_native_token_balance(info: &MessageInfo, fee: Uint128) -> Result<(), ContractError> {
    match info.funds.iter().find(|x| x.denom == String::from("uusd")) {
        Some(coin) => {
            if fee <= coin.amount {
                Ok(())
            } else {
                Err(ContractError::UstBalanceSentToLow {})
            }
        }
        None => {
            if fee.is_zero() {
                Ok(())
            } else {
                Err(ContractError::UstBalanceSentToLow {})
            }
        }
    }
}

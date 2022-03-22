use cosmwasm_std::{
    to_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;

use starterra_token::airdrop_genesis::{AirdropAccount, AirdropInfo};

use crate::errors::ContractError;
use crate::querier::load_token_balance;
use crate::state::{
    read_airdrop_info, read_config, read_pending_owner, remove_pending_owner, store_airdrop_info,
    store_config, store_pending_owner, Config,
};
use crate::tools::{
    assert_sent_native_token_balance, convert_human_to_raw, fetch_user_possible_claim,
    get_ust_withdraw_coin,
};
use std::borrow::BorrowMut;

pub fn claim(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;
    let user_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    assert_sent_native_token_balance(&info.clone(), config.claim_fee)?;

    // If user claimed target, return err
    let airdrop_info = read_airdrop_info(deps.storage, &user_raw)?;
    if airdrop_info.already_claimed >= airdrop_info.amount {
        return Err(ContractError::AlreadyClaimed {});
    }
    let current_possible_claim =
        fetch_user_possible_claim(deps.as_ref(), &user_raw, airdrop_info.amount)?;
    if current_possible_claim <= airdrop_info.already_claimed {
        return Err(ContractError::DoMoreTasks {});
    }

    store_airdrop_info(
        deps.storage.borrow_mut(),
        &user_raw,
        &AirdropInfo {
            amount: airdrop_info.amount,
            already_claimed: current_possible_claim,
        },
    )?;

    let transfer_amount = current_possible_claim - airdrop_info.already_claimed;

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: deps
                .api
                .addr_humanize(&config.starterra_token)?
                .into_string(),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.clone().into_string(),
                amount: transfer_amount,
            })?,
        }))
        .add_attribute("action", "claim")
        .add_attribute("address", info.sender)
        .add_attribute("amount", transfer_amount))
}

pub fn ust_withdraw(deps: DepsMut, env: Env, to: String) -> Result<Response, ContractError> {
    let ust_withdraw_coin = get_ust_withdraw_coin(deps.as_ref(), env)?;
    if ust_withdraw_coin.amount == Uint128::zero() {
        return Err(ContractError::BalanceIsEmpty {});
    }

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: to.clone(),
            amount: vec![ust_withdraw_coin.clone()],
        }))
        .add_attribute("action", "ust_withdraw")
        .add_attribute("recipient", to)
        .add_attribute("ust_withdraw_amount", ust_withdraw_coin.amount))
}

pub fn emergency_withdraw(
    deps: DepsMut,
    env: Env,
    amount: Uint128,
    to: String,
) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;
    let ust_withdraw_coin = get_ust_withdraw_coin(deps.as_ref(), env)?;

    let mut messages = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&config.starterra_token)?
            .into_string(),
        funds: vec![],
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: to.clone(),
            amount,
        })?,
    })];

    if ust_withdraw_coin.amount > Uint128::zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: to.clone(),
            amount: vec![ust_withdraw_coin.clone()],
        }))
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "emergency_withdraw")
        .add_attribute("recipient", to)
        .add_attribute("claim_amount", amount)
        .add_attribute("claim_ust_amount", ust_withdraw_coin.amount))
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

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    owner: Option<String>,
    lp_staking_addresses: Option<Vec<String>>,
    stt_staking_addresses: Option<Vec<String>>,
    ido_addresses: Option<Vec<String>>,
    claim_fee: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;

    if let Some(owner) = owner {
        store_pending_owner(deps.storage, &deps.api.addr_canonicalize(&owner)?)?;
    }

    if let Some(ido_addresses) = ido_addresses {
        config.ido_addresses = convert_human_to_raw(deps.as_ref(), &ido_addresses)?;
    }

    if let Some(lp_staking_addresses) = lp_staking_addresses {
        config.lp_staking_addresses = convert_human_to_raw(deps.as_ref(), &lp_staking_addresses)?;
    }

    if let Some(stt_staking_addresses) = stt_staking_addresses {
        config.stt_staking_addresses = convert_human_to_raw(deps.as_ref(), &stt_staking_addresses)?;
    }

    if let Some(claim_fee) = claim_fee {
        config.claim_fee = claim_fee;
    }

    store_config(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn end_airdrop_genesis(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;
    let ust_withdraw_coin = get_ust_withdraw_coin(deps.as_ref(), env.clone())?;
    let mut messages: Vec<CosmosMsg> = if ust_withdraw_coin.amount > Uint128::zero() {
        vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: deps.api.addr_humanize(&config.owner)?.into_string(),
            amount: vec![ust_withdraw_coin.clone()],
        })]
    } else {
        [].to_vec()
    };

    let token_balance = load_token_balance(
        deps.as_ref(),
        &deps
            .api
            .addr_humanize(&config.starterra_token)?
            .into_string(),
        &deps.api.addr_canonicalize(&env.contract.address.as_str())?,
    )?;

    if !token_balance.is_zero() {
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: deps
                .api
                .addr_humanize(&config.starterra_token)?
                .into_string(),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Burn {
                amount: token_balance,
            })?,
        }));
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "end_airdrop_genesis")
        .add_attribute("burned_tokens_number", token_balance)
        .add_attribute("ust_withdraw_amount", ust_withdraw_coin.amount))
}

pub fn register_airdrop_accounts(
    deps: DepsMut,
    airdrop_accounts: &Vec<AirdropAccount>,
) -> Result<Response, ContractError> {
    for airdrop_account in airdrop_accounts.iter() {
        let airdrop_address = deps.api.addr_canonicalize(&airdrop_account.address)?;
        store_airdrop_info(
            deps.storage,
            &airdrop_address,
            &AirdropInfo {
                already_claimed: airdrop_account.already_claimed,
                amount: airdrop_account.amount,
            },
        )?;
    }

    Ok(Response::new().add_attribute("action", "register_airdrop_accounts"))
}

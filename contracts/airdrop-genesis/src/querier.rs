use cosmwasm_std::{from_binary, to_binary, Binary, CanonicalAddr, QueryRequest, Uint128, WasmQuery, BalanceResponse, BankQuery, Deps};

use cosmwasm_storage::to_length_prefixed;
use starterra_token::staking::{StakerInfoResponse};
use starterra_token::staking::QueryMsg::{StakerInfo};

use crate::errors::ContractError;
use starterra_token::ido::QueryMsg::FunderInfo;
use starterra_token::ido::ParticipantResponse;

pub fn load_token_balance(
    deps: Deps,
    contract_addr: &String,
    account_addr: &CanonicalAddr,
) -> Result<Uint128, ContractError> {
    // load balance form the token contract
    let res: Binary = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Raw {
            contract_addr: String::from(contract_addr),
            key: Binary::from(concat(
                &to_length_prefixed(b"balance").to_vec(),
                account_addr.as_slice(),
            )),
        }))
        .unwrap_or_else(|_| to_binary(&Uint128::zero()).unwrap());

    Ok(from_binary(&res)?)
}

pub fn load_balance(
    deps: Deps,
    account_addr: &String,
    denom: String,
) -> Result<Uint128, ContractError> {
    // load price form the oracle
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: String::from(account_addr),
        denom,
    }))?;
    Ok(balance.amount.amount.into())
}

pub fn check_if_user_stakes(
    deps: Deps,
    contract_addr: &String,
    account_addr: &CanonicalAddr,
) -> Result<bool, ContractError> {
    let res: StakerInfoResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: String::from(contract_addr),
            msg: to_binary(&StakerInfo {
                staker: deps.api.addr_humanize(&account_addr)?.into_string(),
                block_time: None,
            })?,
        })).unwrap_or(StakerInfoResponse {
        staker: "".to_string(),
        reward_index: Default::default(),
        bond_amount: Default::default(),
        pending_reward: Default::default(),
        rewards_per_fee: vec![],
        time_to_best_fee: None,
        pending_unbond_left: None,
        max_submit_to_unbond_amount: None,
        submit_to_unbond_info: None
    });

    return Ok(res.bond_amount > Uint128::zero());
}

pub fn check_if_user_participated_in_ido(
    deps: Deps,
    contract_addr: &String,
    account_addr: &CanonicalAddr,
) -> Result<bool, ContractError> {
    let res: ParticipantResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: String::from(contract_addr),
            msg: to_binary(&FunderInfo {
                address: deps.api.addr_humanize(&account_addr)?.into_string(),
            })?,
        })).unwrap_or(ParticipantResponse {
        is_joined: false,
    });

    return Ok(res.is_joined);
}

#[inline]
fn concat(namespace: &[u8], key: &[u8]) -> Vec<u8> {
    let mut k = namespace.to_vec();
    k.extend_from_slice(key);
    k
}

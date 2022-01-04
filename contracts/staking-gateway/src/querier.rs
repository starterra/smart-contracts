use cosmwasm_std::{to_binary, CanonicalAddr, QueryRequest, StdResult, WasmQuery, Deps};

use starterra_token::staking::QueryMsg::{StakerInfo};
use starterra_token::staking::StakerInfoResponse;

pub fn load_user_staking_status(
    deps: Deps,
    contract_addr: &String,
    account_addr: &CanonicalAddr,
) -> StdResult<(String, StakerInfoResponse)> {
    let res: StakerInfoResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: String::from(contract_addr),
            msg: to_binary(&StakerInfo {
                staker: deps.api.addr_humanize(&account_addr)?.into_string(),
                block_time: None,
            })?,
        }))?;

    return Ok((contract_addr.into(), res));
}

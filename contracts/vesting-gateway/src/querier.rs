use cosmwasm_std::{to_binary, CanonicalAddr, Deps, QueryRequest, StdResult, WasmQuery};

use starterra_token::vesting::common::UserVestingResponse;
use starterra_token::vesting::regular::QueryMsg::UserVesting;

pub fn query_is_address_on_vesting(
    deps: Deps,
    vesting_addr: String,
    account_addr: CanonicalAddr,
) -> StdResult<UserVestingResponse> {
    // check if user is registered on the vesting contract
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: vesting_addr,
        msg: to_binary(&UserVesting {
            address: deps.api.addr_humanize(&account_addr)?.into_string(),
        })?,
    }))
}

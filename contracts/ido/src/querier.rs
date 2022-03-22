use cosmwasm_std::{to_binary, QuerierWrapper, QueryRequest, StdResult, WasmQuery};
use starterra_token::ido_prefund::FunderInfoResponse;
use starterra_token::ido_prefund::QueryMsg::FunderInfo;
use starterra_token::kyc_vault::IsAcceptedVerifiedResponse;
use starterra_token::kyc_vault::QueryMsg::IsAcceptedVerified;

// User balance from ido-prefund
pub fn load_user_prefund_balance(
    querier: &QuerierWrapper,
    contract_addr: String,
    user_address: String,
) -> StdResult<FunderInfoResponse> {
    querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr,
        msg: to_binary(&FunderInfo {
            address: user_address,
        })?,
    }))
}

// Check kyc passed
pub fn check_user_kyc_terms_verified(
    querier: &QuerierWrapper,
    contract_addr: String,
    user_address: String,
) -> StdResult<IsAcceptedVerifiedResponse> {
    querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr,
        msg: to_binary(&IsAcceptedVerified {
            address: user_address,
        })?,
    }))
}

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Coin, ContractResult, Empty, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::testing::mock_querier::QueryMsgMock::{FunderInfo, IsAcceptedVerified};
use starterra_token::ido_prefund::FunderInfoResponse;
use starterra_token::kyc_vault::IsAcceptedVerifiedResponse;
use std::collections::HashMap;
use std::ops::Deref;

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
    contract_address: &str,
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(contract_address, contract_balance)]));

    OwnedDeps {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<Empty>,
    token_querier: TokenQuerier,
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    // this lets us iterate over all pairs that match the first string
    account_info: HashMap<String, HashMap<String, FunderInfoResponse>>,
    kyc_info: HashMap<String, HashMap<String, IsAcceptedVerifiedResponse>>,
}

impl TokenQuerier {
    pub fn new(
        account_info: Vec<(String, Vec<(String, FunderInfoResponse)>)>,
        kyc_info: Vec<(String, Vec<(String, (bool, bool))>)>,
    ) -> Self {
        TokenQuerier {
            account_info: account_info_to_map(account_info),
            kyc_info: account_info_to_terms_map(kyc_info),
        }
    }
}

pub(crate) fn account_info_to_map(
    account_info: Vec<(String, Vec<(String, FunderInfoResponse)>)>,
) -> HashMap<String, HashMap<String, FunderInfoResponse>> {
    let mut account_info_map: HashMap<String, HashMap<String, FunderInfoResponse>> = HashMap::new();
    for (contract_addr, account_info) in account_info.iter() {
        let mut contract_balances_map: HashMap<String, FunderInfoResponse> = HashMap::new();
        for (addr, balance) in account_info.iter() {
            contract_balances_map.insert(addr.clone(), balance.deref().clone());
        }

        account_info_map.insert(contract_addr.clone(), contract_balances_map);
    }
    account_info_map
}

pub(crate) fn account_info_to_terms_map(
    account_info: Vec<(String, Vec<(String, (bool, bool))>)>,
) -> HashMap<String, HashMap<String, IsAcceptedVerifiedResponse>> {
    let mut account_info_map: HashMap<String, HashMap<String, IsAcceptedVerifiedResponse>> =
        HashMap::new();
    for (contract_addr, account_info) in account_info.iter() {
        let mut contract_balances_map: HashMap<String, IsAcceptedVerifiedResponse> = HashMap::new();
        for (addr, info) in account_info.iter() {
            contract_balances_map.insert(
                addr.clone(),
                IsAcceptedVerifiedResponse {
                    address: String::from(addr),
                    is_accepted: info.0,
                    is_verified: info.1,
                },
            );
        }

        account_info_map.insert(contract_addr.clone(), contract_balances_map);
    }
    account_info_map
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                });
            }
        };
        self.handle_query(&request)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsgMock {
    IsAcceptedVerified { address: String },
    FunderInfo { address: String },
    Config {},
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                let msg = from_binary(&msg).unwrap();
                match msg {
                    FunderInfo { address } => {
                        let funder_info_response = &FunderInfoResponse {
                            available_funds: Uint128::zero(),
                            spent_funds: Uint128::zero(),
                        };
                        let map_for_contract = self
                            .token_querier
                            .account_info
                            .get(contract_addr.clone().as_str())
                            .unwrap();
                        let resp = map_for_contract
                            .get(address.as_str())
                            .unwrap_or_else(|| &funder_info_response);
                        SystemResult::Ok(ContractResult::from(to_binary(&resp)))
                    }
                    IsAcceptedVerified { address } => {
                        let is_accepted_response = &IsAcceptedVerifiedResponse {
                            address: address.clone(),
                            is_accepted: false,
                            is_verified: false,
                        };
                        let map_for_contract = self
                            .token_querier
                            .kyc_info
                            .get(contract_addr.clone().as_str())
                            .unwrap();
                        let resp = map_for_contract
                            .get(address.as_str())
                            .unwrap_or_else(|| &is_accepted_response);
                        SystemResult::Ok(ContractResult::from(to_binary(&resp)))
                    }

                    _ => self.base.handle_query(request),
                }
            }
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<Empty>) -> Self {
        WasmMockQuerier {
            base,
            token_querier: TokenQuerier::default(),
        }
    }

    // configure the mint whitelist mock querier
    pub fn with_account_statuses(
        &mut self,
        account_statuses: Vec<(String, Vec<(String, FunderInfoResponse)>)>,
        kyc_terms_statuses: Vec<(String, Vec<(String, (bool, bool))>)>,
    ) {
        self.token_querier = TokenQuerier::new(account_statuses, kyc_terms_statuses);
    }
}

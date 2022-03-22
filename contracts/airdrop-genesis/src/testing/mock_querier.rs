use crate::testing::mock_querier::QueryMsgMock::{FunderInfo, StakerInfo};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Coin, ContractResult, Empty, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, WasmQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use starterra_token::ido::ParticipantResponse;
use starterra_token::staking::StakerInfoResponse;
use std::collections::HashMap;

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

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
    staker_info: HashMap<String, HashMap<String, StakerInfoResponse>>,
    ido_participant_info: HashMap<String, HashMap<String, ParticipantResponse>>,
}

impl TokenQuerier {
    pub fn new(
        staker_info: Vec<(String, Vec<(String, StakerInfoResponse)>)>,
        funder_info: Vec<(String, Vec<(String, ParticipantResponse)>)>,
    ) -> Self {
        TokenQuerier {
            staker_info: data_to_map(staker_info),
            ido_participant_info: data_to_map(funder_info),
        }
    }
}

pub(crate) fn data_to_map<T>(
    staker_info: Vec<(String, Vec<(String, T)>)>,
) -> HashMap<String, HashMap<String, T>> {
    let mut staker_info_map: HashMap<String, HashMap<String, T>> = HashMap::new();
    for (contract_addr, staker_info) in staker_info {
        let mut contract_balances_map: HashMap<String, T> = HashMap::new();
        for (addr, balance) in staker_info {
            contract_balances_map.insert(String::from(addr), balance);
        }

        staker_info_map.insert(String::from(contract_addr), contract_balances_map);
    }
    staker_info_map
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
    StakerInfo {
        staker: String,
        block_time: Option<u64>,
    },
    FunderInfo {
        address: String,
    },
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                let msg = from_binary(&msg).unwrap();
                match msg {
                    StakerInfo {
                        staker: address,
                        block_time: _none,
                    } => {
                        let staking_information: &HashMap<String, StakerInfoResponse> =
                            match self.token_querier.staker_info.get(contract_addr) {
                                Some(staker_info) => staker_info,
                                None => {
                                    return SystemResult::Err(SystemError::InvalidRequest {
                                        error: format!(
                                            "No staking info exists for the contract {}",
                                            contract_addr
                                        ),
                                        request: Default::default(),
                                    });
                                }
                            };
                        let value = match staking_information.get(&address) {
                            Some(v) => v,
                            None => {
                                return SystemResult::Err(SystemError::InvalidRequest {
                                    error: "Value not found".to_string(),
                                    request: Default::default(),
                                });
                            }
                        };

                        SystemResult::Ok(ContractResult::from(to_binary(&StakerInfoResponse {
                            staker: address,
                            reward_index: value.reward_index,
                            bond_amount: value.bond_amount,
                            pending_reward: value.pending_reward,
                            rewards_per_fee: vec![],
                            time_to_best_fee: None,
                            pending_unbond_left: None,
                            max_submit_to_unbond_amount: None,
                            submit_to_unbond_info: None,
                        })))
                    }
                    FunderInfo { address } => {
                        let funders_information: &HashMap<String, ParticipantResponse> =
                            match self.token_querier.ido_participant_info.get(contract_addr) {
                                Some(funder_info) => funder_info,
                                None => {
                                    return SystemResult::Err(SystemError::InvalidRequest {
                                        error: format!(
                                            "No funder info exists for the contract {}",
                                            contract_addr
                                        ),
                                        request: Default::default(),
                                    });
                                }
                            };
                        let value = match funders_information.get(&address) {
                            Some(v) => v,
                            None => {
                                return SystemResult::Err(SystemError::InvalidRequest {
                                    error: "Value not found".to_string(),
                                    request: Default::default(),
                                });
                            }
                        };

                        SystemResult::Ok(ContractResult::from(to_binary(&ParticipantResponse {
                            is_joined: value.is_joined,
                        })))
                    }
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
    pub fn with_staker_info(
        &mut self,
        staker_info: Vec<(String, Vec<(String, StakerInfoResponse)>)>,
        funder_info: Vec<(String, Vec<(String, ParticipantResponse)>)>,
    ) {
        self.token_querier = TokenQuerier::new(staker_info, funder_info);
    }
}

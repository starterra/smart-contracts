use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Coin, ContractResult, Decimal, Empty, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use std::collections::HashMap;

use starterra_token::staking::{StakerInfo, StakerInfoResponse};

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
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
    staker_info: HashMap<String, HashMap<String, StakerInfo>>,
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

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                let msg = from_binary(&msg).unwrap();
                match msg {
                    starterra_token::staking::QueryMsg::StakerInfo {
                        staker: address,
                        block_time: _none,
                    } => {
                        let staking_information: &HashMap<String, StakerInfo> =
                            match self.token_querier.staker_info.get(contract_addr) {
                                Some(staker_info) => staker_info,
                                None => {
                                    return SystemResult::Ok(ContractResult::from(to_binary(
                                        &StakerInfoResponse {
                                            staker: address,
                                            reward_index: Decimal::zero(),
                                            bond_amount: Uint128::zero(),
                                            pending_reward: Uint128::zero(),
                                            rewards_per_fee: vec![],
                                            time_to_best_fee: None,
                                            pending_unbond_left: None,
                                            max_submit_to_unbond_amount: None,
                                            submit_to_unbond_info: None,
                                        },
                                    )))
                                }
                            };
                        let value = match staking_information.get(&address) {
                            Some(v) => v,
                            None => {
                                return SystemResult::Ok(ContractResult::from(to_binary(
                                    &StakerInfoResponse {
                                        staker: address,
                                        reward_index: Decimal::zero(),
                                        bond_amount: Uint128::zero(),
                                        pending_reward: Uint128::zero(),
                                        rewards_per_fee: vec![],
                                        time_to_best_fee: None,
                                        pending_unbond_left: None,
                                        max_submit_to_unbond_amount: None,
                                        submit_to_unbond_info: None,
                                    },
                                )))
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
    pub fn with_staker_info(&mut self, staker_info: Vec<(String, Vec<(String, StakerInfo)>)>) {
        self.token_querier.staker_info = data_to_map(staker_info);
    }
}

use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{attr, from_binary, Timestamp, Uint128};

use starterra_token::ido::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, ParticipantResponse, ParticipantsResponse,
    QueryMsg, StateResponse, StatusResponse,
};
use starterra_token::ido_prefund::FunderInfoResponse;

use crate::contract::{execute, instantiate, query};
use crate::errors::ContractError;
use crate::testing::mock_querier::mock_dependencies;
use starterra_token::common::OrderBy;

#[test]
fn proper_instantiate() {
    let mut deps = mock_dependencies(&[], "some");
    let info = mock_info("newaddr", &vec![]);
    let msg = InstantiateMsg {
        owner: String::from("newaddr"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 100000u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<ConfigResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()
        )
        .unwrap(),
        ConfigResponse {
            owner: String::from("newaddr".to_string()),
            prefund_address: String::from("prefund_addr".to_string()),
            kyc_terms_vault_address: String::from("kyc_vault_address".to_string()),
            ido_token: String::from("ido_token_address".to_string()),
            ido_token_price: Uint128::from(100u128),
            end_date: 100000u64,
            paused: false,
            snapshot_time: None,
            minimum_prefund: Uint128::from(500u128),
        }
    );
}

#[test]
fn instantiate_with_end_date_in_the_past() {
    let mut deps = mock_dependencies(&[], "some");
    let info = mock_info("newaddr", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let msg = InstantiateMsg {
        owner: String::from("newaddr"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 99u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    assert_eq!(res, ContractError::EndDateInThePast {})
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies(&[], "some");

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 100000u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("new_owner")),
        prefund_address: Some(String::from("prefund_address")),
        kyc_terms_vault_address: Some(String::from("kyc_vault_address")),
        ido_token: Some(String::from("new_ido_token")),
        ido_token_price: Some(Uint128::from(200u128)),
        end_date: Some(200000u64),
        paused: Some(true),
        snapshot_time: Some(100u64),
        minimum_prefund: Some(Uint128::from(321321u128)),
    };
    let info = mock_info("owner", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(10u64);
    let _res = execute(deps.as_mut(), env, info.clone(), msg).unwrap();

    let msg = ExecuteMsg::AcceptOwnership {};
    let info = mock_info("new_owner", &vec![]);
    let env = mock_env();
    let _res = execute(deps.as_mut(), env, info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<ConfigResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()
        )
        .unwrap(),
        ConfigResponse {
            owner: String::from("new_owner".to_string()),
            prefund_address: String::from("prefund_address".to_string()),
            kyc_terms_vault_address: String::from("kyc_vault_address".to_string()),
            ido_token: String::from("new_ido_token".to_string()),
            ido_token_price: Uint128::from(200u128),
            end_date: 200000u64,
            paused: true,
            snapshot_time: Some(100u64),
            minimum_prefund: Uint128::from(321321u128),
        }
    );

    assert_eq!(
        from_binary::<StatusResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::Status {
                    block_time: Some(200001u64)
                }
            )
            .unwrap()
        )
        .unwrap(),
        StatusResponse {
            is_paused: true,
            is_closed: true,
            snapshot_time: Some(100u64),
        }
    );

    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("new_owner3")),
        prefund_address: Some(String::from("prefund_address3")),
        kyc_terms_vault_address: Some(String::from("kyc_vault_address3")),
        ido_token: Some(String::from("new_ido_token_address3")),
        ido_token_price: Some(Uint128::from(2001u128)),
        end_date: Some(200000u64),
        paused: Some(false),
        snapshot_time: None,
        minimum_prefund: Some(Uint128::from(321321u128)),
    };
    let info = mock_info("owner", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let env = mock_env();
    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("new_owner")),
        prefund_address: Some(String::from("prefund_address")),
        kyc_terms_vault_address: Some(String::from("kyc_vault_address")),
        ido_token: Some(String::from("new_ido_token")),
        ido_token_price: Some(Uint128::from(200u128)),
        end_date: Some(env.block.time.seconds() - 1),
        paused: Some(true),
        snapshot_time: Some(100u64),
        minimum_prefund: Some(Uint128::from(321321u128)),
    };
    let info = mock_info("new_owner", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    match res {
        Err(ContractError::EndDateInThePast { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

#[test]
fn accept_ownership() {
    let mut deps = mock_dependencies(&[], "some");

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 100000u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    let msg = ExecuteMsg::AcceptOwnership {};
    let info = mock_info("new_owner", &vec![]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info.clone(), msg);
    match res {
        Err(ContractError::PendingOwnerMissing { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("new_owner")),
        prefund_address: None,
        kyc_terms_vault_address: None,
        ido_token: None,
        ido_token_price: None,
        end_date: None,
        paused: None,
        snapshot_time: None,
        minimum_prefund: None,
    };
    let info = mock_info("owner", &vec![]);
    let env = mock_env();
    let _res = execute(deps.as_mut(), env, info.clone(), msg).unwrap();

    let msg = ExecuteMsg::AcceptOwnership {};
    let info = mock_info("attacker", &vec![]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info.clone(), msg);
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let msg = ExecuteMsg::AcceptOwnership {};
    let info = mock_info("new_owner", &vec![]);
    let env = mock_env();
    let res = execute(deps.as_mut(), env, info.clone(), msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "accept_ownership"),
            attr("owner", info.sender),
        ]
    );
}

#[test]
fn join_ido() {
    let mut deps = mock_dependencies(&[], "some");

    deps.querier.with_account_statuses(
        vec![(
            String::from("prefund_addr"),
            vec![(
                String::from("ido_address_1"),
                FunderInfoResponse {
                    available_funds: Uint128::from(500u128),
                    spent_funds: Uint128::zero(),
                },
            )],
        )],
        vec![
            ((
                String::from("kyc_vault_address"),
                vec![(String::from("ido_address_1"), (true, true))],
            )),
        ],
    );

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 100000u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };

    let info = mock_info("addr0000", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    // not enough prefund
    let msg = ExecuteMsg::JoinIdo {};
    let info = mock_info("ido_address_2", &[]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let res = execute(deps.as_mut(), env, info.clone(), msg);
    match res {
        Err(ContractError::NotEnoughDeposit { .. }) => {}
        _ => panic!("Must return You have to deposit more on prefund contract"),
    }

    // join ido success
    let msg = ExecuteMsg::JoinIdo {};
    let info2 = mock_info("ido_address_1", &[]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let res = execute(deps.as_mut(), env, info2.clone(), msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "join_ido"), attr("address", "ido_address_1"),]
    );

    assert_eq!(
        from_binary::<ParticipantResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::FunderInfo {
                    address: String::from("ido_address_1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        ParticipantResponse { is_joined: true }
    );

    assert_eq!(
        from_binary::<ParticipantResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::FunderInfo {
                    address: String::from("ido_address_2"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        ParticipantResponse { is_joined: false }
    );

    assert_eq!(
        from_binary::<StateResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::State {},).unwrap()
        )
        .unwrap(),
        StateResponse {
            number_of_participants: 1,
        }
    );
}

#[test]
fn cant_join_ido_after_end_time() {
    let mut deps = mock_dependencies(&[], "some");

    deps.querier.with_account_statuses(
        vec![(
            String::from("prefund_addr"),
            vec![(
                String::from("ido_address_1"),
                FunderInfoResponse {
                    available_funds: Uint128::from(100u128),
                    spent_funds: Uint128::zero(),
                },
            )],
        )],
        vec![
            ((
                String::from("kyc_vault_address"),
                vec![(String::from("ido_address_1"), (true, true))],
            )),
        ],
    );

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 100000u64,
        paused: true,
        minimum_prefund: Uint128::from(500u128),
    };

    let info = mock_info("addr0000", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    let msg = ExecuteMsg::JoinIdo {};
    let info = mock_info("ido_address_1", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

    match res {
        Err(ContractError::IdoPaused { .. }) => {}
        _ => panic!("Must return IDO is paused"),
    }

    assert_eq!(
        from_binary::<StatusResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::Status {
                    block_time: Some(10u64)
                }
            )
            .unwrap()
        )
        .unwrap(),
        StatusResponse {
            is_paused: true,
            is_closed: false,
            snapshot_time: None,
        }
    );

    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        prefund_address: None,
        kyc_terms_vault_address: None,
        ido_token: None,
        ido_token_price: None,
        end_date: Some(100000u64),
        paused: Some(false),
        snapshot_time: None,
        minimum_prefund: None,
    };
    let info = mock_info("owner", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let _res = execute(deps.as_mut(), env, info.clone(), msg).unwrap();

    let msg = ExecuteMsg::JoinIdo {};
    let info = mock_info("ido_address_1", &[]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100001u64);
    let res = execute(deps.as_mut(), env, info.clone(), msg);

    match res {
        Err(ContractError::IdoClosed { .. }) => {}
        _ => panic!("Must return IDO is closed"),
    }

    assert_eq!(
        from_binary::<StatusResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::Status {
                    block_time: Some(100001u64)
                }
            )
            .unwrap()
        )
        .unwrap(),
        StatusResponse {
            is_paused: false,
            is_closed: true,
            snapshot_time: None,
        }
    );
}

#[test]
fn cant_join_ido_second_time() {
    let mut deps = mock_dependencies(&[], "some");

    deps.querier.with_account_statuses(
        vec![(
            String::from("prefund_addr"),
            vec![(
                String::from("ido_address_1"),
                FunderInfoResponse {
                    available_funds: Uint128::from(500u128),
                    spent_funds: Uint128::zero(),
                },
            )],
        )],
        vec![
            ((
                String::from("kyc_vault_address"),
                vec![(String::from("ido_address_1"), (true, true))],
            )),
        ],
    );

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let msg = InstantiateMsg {
        owner: String::from("owner"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 100000u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    // join ido
    let msg = ExecuteMsg::JoinIdo {};
    let info = mock_info("ido_address_1", &[]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(10000u64);
    let _res = execute(deps.as_mut(), env, info.clone(), msg).unwrap();

    // join second time
    let msg = ExecuteMsg::JoinIdo {};
    let info = mock_info("ido_address_1", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

    match res {
        Err(ContractError::AlreadyJoined { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

#[test]
fn query_snapshot() {
    let mut deps = mock_dependencies(&[], "some");

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let msg = InstantiateMsg {
        owner: String::from("owner"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 1000000u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<Option<u64>>(
            &query(deps.as_ref(), mock_env(), QueryMsg::SnapshotTime {}).unwrap()
        )
        .unwrap(),
        None
    );

    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        prefund_address: None,
        kyc_terms_vault_address: None,
        ido_token: None,
        ido_token_price: None,
        end_date: None,
        paused: None,
        snapshot_time: Some(2000000u64),
        minimum_prefund: None,
    };

    let info = mock_info("owner", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(10000u64);
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<Option<u64>>(
            &query(deps.as_ref(), mock_env(), QueryMsg::SnapshotTime {}).unwrap()
        )
        .unwrap(),
        Some(2000000u64)
    );

    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        prefund_address: None,
        kyc_terms_vault_address: None,
        ido_token: None,
        ido_token_price: None,
        end_date: None,
        paused: None,
        snapshot_time: None,
        minimum_prefund: None,
    };
    let info = mock_info("owner", &vec![]);
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<Option<u64>>(
            &query(deps.as_ref(), mock_env(), QueryMsg::SnapshotTime {}).unwrap()
        )
        .unwrap(),
        Some(2000000u64)
    );

    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        prefund_address: None,
        kyc_terms_vault_address: None,
        ido_token: None,
        ido_token_price: None,
        end_date: None,
        paused: None,
        snapshot_time: Some(10u64),
        minimum_prefund: None,
    };
    let info = mock_info("owner", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100u64);
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    match res {
        ContractError::SnapshotTimeFromPast { .. } => {}
        _ => panic!("Must return Snapshot from the past"),
    }
}

#[test]
fn query_participants() {
    let mut deps = mock_dependencies(&[], "some");

    deps.querier.with_account_statuses(
        vec![(
            String::from("prefund_addr"),
            vec![
                (
                    String::from("ido_address_1"),
                    FunderInfoResponse {
                        available_funds: Uint128::from(500u128),
                        spent_funds: Uint128::zero(),
                    },
                ),
                (
                    String::from("ido_address_2"),
                    FunderInfoResponse {
                        available_funds: Uint128::from(500u128),
                        spent_funds: Uint128::zero(),
                    },
                ),
                (
                    String::from("ido_address_3"),
                    FunderInfoResponse {
                        available_funds: Uint128::from(500u128),
                        spent_funds: Uint128::zero(),
                    },
                ),
            ],
        )],
        vec![
            ((
                String::from("kyc_vault_address"),
                vec![
                    (String::from("ido_address_1"), (true, true)),
                    (String::from("ido_address_2"), (true, true)),
                    (String::from("ido_address_3"), (true, true)),
                ],
            )),
        ],
    );

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        prefund_address: String::from("prefund_addr"),
        kyc_terms_vault_address: String::from("kyc_vault_address"),
        ido_token: String::from("ido_token_address"),
        ido_token_price: Uint128::from(100u128),
        end_date: 100000u64,
        paused: false,
        minimum_prefund: Uint128::from(500u128),
    };

    let info = mock_info("addr0000", &vec![]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    // join ido success
    let msg = ExecuteMsg::JoinIdo {};
    let info2 = mock_info("ido_address_1", &[]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    execute(deps.as_mut(), env, info2.clone(), msg).unwrap();

    let msg = ExecuteMsg::JoinIdo {};
    let info2 = mock_info("ido_address_2", &[]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    execute(deps.as_mut(), env, info2.clone(), msg).unwrap();

    let msg = ExecuteMsg::JoinIdo {};
    let info2 = mock_info("ido_address_3", &[]);
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    execute(deps.as_mut(), env, info2.clone(), msg).unwrap();

    //query for participants
    let msg = QueryMsg::Participants {
        start_after: None,
        limit: None,
        order_by: Some(OrderBy::Asc),
    };
    let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    let participants: ParticipantsResponse = from_binary(&res).unwrap();
    assert_eq!(
        participants.users,
        vec!["ido_address_1", "ido_address_2", "ido_address_3"]
    );

    //query for participants with limit
    let msg = QueryMsg::Participants {
        start_after: None,
        limit: Some(2),
        order_by: Some(OrderBy::Asc),
    };
    let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    let participants: ParticipantsResponse = from_binary(&res).unwrap();
    assert_eq!(participants.users, vec!["ido_address_1", "ido_address_2"]);

    //query for participants with limit
    let msg = QueryMsg::Participants {
        start_after: Some("ido_address_2".to_string()),
        limit: Some(2),
        order_by: Some(OrderBy::Asc),
    };
    let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    let participants: ParticipantsResponse = from_binary(&res).unwrap();
    assert_eq!(participants.users, vec!["ido_address_3"]);
}

use cosmwasm_std::{attr, BankMsg, Coin, coin, CosmosMsg, from_binary, SubMsg, to_binary, Uint128, WasmMsg};
use cosmwasm_std::testing::{mock_env, mock_info};
use cw20::Cw20ExecuteMsg;

use starterra_token::airdrop_genesis::{AirdropAccount, AirdropUserInfoResponse, ConfigResponse, ExecuteMsg, InstantiateMsg, PassedMissions, QueryMsg};
use starterra_token::staking::StakerInfoResponse;

use crate::contract::{execute, instantiate, query};
use crate::errors::ContractError;
use crate::testing::mock_querier::mock_dependencies;
use starterra_token::ido::ParticipantResponse;

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![String::from("addr1"), String::from("addr2")],
        stt_staking_addresses: vec![String::from("addrstt1"), String::from("addrstt2")],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // it worked, let's query the state
    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        config,
        ConfigResponse {
            owner: String::from("owner"),
            starterra_token: String::from("starterra"),
            lp_staking_addresses: vec![String::from("addr1"), String::from("addr2")],
            stt_staking_addresses: vec![String::from("addrstt1"), String::from("addrstt2")],
            ido_addresses: vec![],
        }
    );
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // update owner
    let env = mock_env();
    let info = mock_info("owner2", &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("owner2")),
        lp_staking_addresses: None,
        stt_staking_addresses: None,
        ido_addresses: None,
        claim_fee: None,
    };

    let res = execute(deps.as_mut(), env, info, msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = ExecuteMsg::AcceptOwnership {};
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg);
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let msg = ExecuteMsg::AcceptOwnership {};
    let env = mock_env();
    let info = mock_info("owner2", &[]);
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // it worked, let's query the state
    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!("owner2", config.owner.as_str());

    // Unauthorzied err
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        lp_staking_addresses: None,
        stt_staking_addresses: None,
        ido_addresses: None,
        claim_fee: None,
    };

    let res = execute(deps.as_mut(), env, info, msg);
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }

    //update staking and ido addresses
    let env = mock_env();
    let info = mock_info("owner2", &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        lp_staking_addresses: Some(vec![String::from("staking1"), String::from("staking2")]),
        stt_staking_addresses: Some(vec![String::from("stakingstt1"), String::from("stakingstt2")]),
        ido_addresses: Some(vec![String::from("ido1"), String::from("ido2")]),
        claim_fee: Some(Uint128::from(1000000u128)),
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!("owner2", config.owner.as_str());
    assert_eq!(vec![String::from("staking1"), String::from("staking2")], config.lp_staking_addresses);
    assert_eq!(vec![String::from("stakingstt1"), String::from("stakingstt2")], config.stt_staking_addresses);
    assert_eq!(vec![String::from("ido1"), String::from("ido2")], config.ido_addresses);
}

#[test]
fn accept_ownership() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };
    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

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
        lp_staking_addresses: None,
        stt_staking_addresses: None,
        ido_addresses: None,
        claim_fee: None,
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
fn claim() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    // Register airdrop accounts
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(1000000u128),
            already_claimed: Uint128::from(0u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        }]
    };

    let _res = execute(deps.as_mut(), env, info, msg).unwrap();

    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from("starterra"),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                amount: Uint128::from(250000u128),
            })
                .unwrap(),
        }))]
    );

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "claim"),
            attr("address", "terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
            attr("amount", "250000"),
        ]
    );

    assert_eq!(
        Uint128::from(250000u128),
        from_binary::<AirdropUserInfoResponse>(
            &query(
                deps.as_ref(),
                env,
                QueryMsg::UserInfo {
                    address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                },
            )
                .unwrap()
        )
            .unwrap()
            .claimed_amount
    );

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(2000000u128),
            already_claimed: Uint128::from(0u128),
            address: String::from("terra1tjv5e0lr5s3fum4wfj7grtclm34xvms5dv9l75"),
        }]
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    // Claim after registering airdrop account update
    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1tjv5e0lr5s3fum4wfj7grtclm34xvms5dv9l75",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from("starterra"),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: String::from("terra1tjv5e0lr5s3fum4wfj7grtclm34xvms5dv9l75"),
                amount: Uint128::from(500000u128),
            })
                .unwrap(),
        }))]
    );

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "claim"),
            attr("address", "terra1tjv5e0lr5s3fum4wfj7grtclm34xvms5dv9l75"),
            attr("amount", "500000"),
        ]
    );
}

#[test]
fn double_claim_should_reject() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    // Register airdrop accounts
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(1000000u128),
            already_claimed: Uint128::from(0u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        }]
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();

    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);

    execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    match res {
        Err(ContractError::DoMoreTasks {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

#[test]
fn double_claim_after_staking_should_not_reject() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![String::from("staking1"), String::from("staking2"), String::from("staking3")],
        stt_staking_addresses: vec![String::from("stakingstt1"), String::from("stakingstt2"), String::from("stakingstt3")],
        ido_addresses: vec![String::from("ido1"), String::from("ido2")],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    // Register airdrop accounts
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(1000000u128),
            already_claimed: Uint128::from(0u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        },
             AirdropAccount {
                 amount: Uint128::from(1000000u128),
                 already_claimed: Uint128::from(0u128),
                 address: String::from("terra1csnmlw0v0pyy36tk7scfwvh8ujpnydu5dtfj58"),
             }]
    };

    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let res = query(
        deps.as_ref(),
        env.clone(),
        QueryMsg::UserInfo {
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8")
        }).unwrap();
    let claimed_amount: AirdropUserInfoResponse = from_binary(&res).unwrap();
    assert_eq!(
        claimed_amount,
        AirdropUserInfoResponse {
            claimed_amount: Uint128::zero(),
            initial_claim_amount: Uint128::from(1000000u128),
            current_passed_missions: PassedMissions {
                is_in_lp_staking: false,
                is_in_stt_staking: false,
                is_in_ido: false,
            },
        }
    );

    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);
    execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

    let res = query(deps.as_ref(), env.clone(), QueryMsg::UserInfo { address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8") }).unwrap();
    let claimed_amount: AirdropUserInfoResponse = from_binary(&res).unwrap();
    assert_eq!(
        claimed_amount,
        AirdropUserInfoResponse {
            claimed_amount: Uint128::from(250000u128),
            initial_claim_amount: Uint128::from(1000000u128),
            current_passed_missions: PassedMissions {
                is_in_lp_staking: false,
                is_in_stt_staking: false,
                is_in_ido: false,
            },
        }
    );

    deps.querier.with_staker_info(
        vec![
            (
                String::from("staking2"),
                vec![(
                    String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                    StakerInfoResponse {
                        staker: "".to_string(),
                        reward_index: Default::default(),
                        bond_amount: Uint128::from(100u64),
                        pending_reward: Default::default(),
                        rewards_per_fee: vec![],
                        time_to_best_fee: None,
                        pending_unbond_left: None,
                        max_submit_to_unbond_amount: None,
                        submit_to_unbond_info: None,
                    }
                )],
            ),
            (
                String::from("stakingstt2"),
                vec![(
                         String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                         StakerInfoResponse {
                             staker: "".to_string(),
                             reward_index: Default::default(),
                             bond_amount: Uint128::from(100u64),
                             pending_reward: Default::default(),
                             rewards_per_fee: vec![],
                             time_to_best_fee: None,
                             pending_unbond_left: None,
                             max_submit_to_unbond_amount: None,
                             submit_to_unbond_info: None,
                         }
                     ),
                     (
                         String::from("terra1csnmlw0v0pyy36tk7scfwvh8ujpnydu5dtfj58"),
                         StakerInfoResponse {
                             staker: "".to_string(),
                             reward_index: Default::default(),
                             bond_amount: Uint128::from(100u64),
                             pending_reward: Default::default(),
                             rewards_per_fee: vec![],
                             time_to_best_fee: None,
                             pending_unbond_left: None,
                             max_submit_to_unbond_amount: None,
                             submit_to_unbond_info: None,
                         }
                     )],
            ),
        ],
        vec![
            (
                String::from("ido1"),
                vec![(
                    String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                    ParticipantResponse {
                        is_joined: true
                    }
                )],
            ),
            (
                String::from("ido2"),
                vec![(
                    String::from("terra1csnmlw0v0pyy36tk7scfwvh8ujpnydu5dtfj58"),
                    ParticipantResponse {
                        is_joined: true
                    }
                )],
            )
        ],
    );

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from("starterra"),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                amount: Uint128::from(750000u128),
            })
                .unwrap(),
        }))]
    );

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "claim"),
            attr("address", "terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
            attr("amount", "750000"),
        ]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::UserInfo { address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8") }).unwrap();
    let claimed_amount: AirdropUserInfoResponse = from_binary(&res).unwrap();
    assert_eq!(
        claimed_amount,
        AirdropUserInfoResponse {
            claimed_amount: Uint128::from(1000000u128),
            initial_claim_amount: Uint128::from(1000000u128),
            current_passed_missions: PassedMissions {
                is_in_lp_staking: true,
                is_in_stt_staking: true,
                is_in_ido: true,
            },
        }
    );

    let res2 = query(deps.as_ref(), env, QueryMsg::UserInfo { address: String::from("terra1csnmlw0v0pyy36tk7scfwvh8ujpnydu5dtfj58") }).unwrap();
    let claimed_amount_user2: AirdropUserInfoResponse = from_binary(&res2).unwrap();
    assert_eq!(
        claimed_amount_user2.current_passed_missions,
        PassedMissions {
            is_in_lp_staking: false,
            is_in_stt_staking: true,
            is_in_ido: true,
        }
    );
}

#[test]
fn claim_without_fee_should_reject() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    // Register airdrop accounts
    let env = mock_env();
    let info = mock_info("not_owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(1000000u128),
            already_claimed: Uint128::from(0u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        }]
    };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();

    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8", &[]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    match res {
        Err(ContractError::UstBalanceSentToLow {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

#[test]
fn end_airdrop_genesis() {
    let mut deps = mock_dependencies(&[coin(100, "uusd")]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    let env = mock_env();
    let info = mock_info("owner2", &[]);
    let msg = ExecuteMsg::EndGenesisAirdrop {};

    let res = execute(deps.as_mut(), env, info, msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::EndGenesisAirdrop {};
    let _res = execute(deps.as_mut(), env, info, msg.clone());

    let env = mock_env();
    let info = mock_info("owner", &[]);

    let _res = execute(deps.as_mut(), env, info, msg.clone()).unwrap();

    let env = mock_env();
    let info = mock_info("owner", &[]);

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res.messages,
        [SubMsg::new(CosmosMsg::Bank(
            BankMsg::Send {
                to_address: String::from("owner"),
                amount: vec![Coin { denom: "uusd".to_string(), amount: Uint128::from(100u128) }],
            },
        ))]
    );

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "end_airdrop_genesis"),
            attr("burned_tokens_number", "0"),
            attr("ust_withdraw_amount", "100"),
        ]
    );
}

#[test]
fn emergency_withdraw() {
    let mut deps = mock_dependencies(&vec![coin(100, "uusd")]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra_token"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // No permissions
    let env = mock_env();
    let info = mock_info("otherAddr", &[]);
    let msg = ExecuteMsg::EmergencyWithdraw {
        amount: Uint128::from(1000u128),
        to: String::from("otherAddr"),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    // Success withdraw
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::EmergencyWithdraw {
        amount: Uint128::from(1000u128),
        to: String::from("owner"),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "emergency_withdraw"),
            attr("recipient", "owner"),
            attr("claim_amount", "1000"),
            attr("claim_ust_amount", "100"),
        ]
    );
    assert_eq!(
        res.messages,
        vec![
            SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: String::from("starterra_token"),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: String::from("owner"),
                    amount: Uint128::from(1000u128),
                })
                    .unwrap(),
                funds: vec![],
            })),
            SubMsg::new(CosmosMsg::Bank(
                BankMsg::Send {
                    to_address: String::from("owner"),
                    amount: vec![coin(100, "uusd")],
                },
            )),
        ],
    );
}

#[test]
fn claim_with_second_register() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    // Register airdrop accounts
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(1000000u128),
            already_claimed: Uint128::from(0u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        }]
    };

    let _res = execute(deps.as_mut(), env, info, msg).unwrap();

    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from("starterra"),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                amount: Uint128::from(250000u128),
            })
                .unwrap(),
        }))]
    );

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "claim"),
            attr("address", "terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
            attr("amount", "250000"),
        ]
    );

    assert_eq!(
        Uint128::from(250000u128),
        from_binary::<AirdropUserInfoResponse>(
            &query(
                deps.as_ref(),
                env,
                QueryMsg::UserInfo {
                    address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                },
            )
                .unwrap()
        )
            .unwrap()
            .claimed_amount
    );

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(2000000u128),
            already_claimed: Uint128::from(250000u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        }]
    };

    let _res = execute(deps.as_mut(), env, info, msg).unwrap();


    // Claim after registering airdrop account update
    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from("starterra"),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                amount: Uint128::from(250000u128),
            })
                .unwrap(),
        }))]
    );

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "claim"),
            attr("address", "terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
            attr("amount", "250000"),
        ]
    );
}

#[test]
fn claim_with_second_register_over_limit() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        starterra_token: String::from("starterra"),
        lp_staking_addresses: vec![],
        stt_staking_addresses: vec![],
        ido_addresses: vec![],
        claim_fee: Uint128::from(1000000u128),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    // Register airdrop accounts
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(1000000u128),
            already_claimed: Uint128::from(0u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        }]
    };

    let _res = execute(deps.as_mut(), env, info, msg).unwrap();

    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from("starterra"),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                amount: Uint128::from(250000u128),
            })
                .unwrap(),
        }))]
    );

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "claim"),
            attr("address", "terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
            attr("amount", "250000"),
        ]
    );

    assert_eq!(
        Uint128::from(250000u128),
        from_binary::<AirdropUserInfoResponse>(
            &query(
                deps.as_ref(),
                env,
                QueryMsg::UserInfo {
                    address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
                },
            )
                .unwrap()
        )
            .unwrap()
            .claimed_amount
    );

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RegisterAirdropAccounts {
        airdrop_accounts:
        vec![AirdropAccount {
            amount: Uint128::from(500000u128),
            already_claimed: Uint128::from(250000u128),
            address: String::from("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8"),
        }]
    };

    let _res = execute(deps.as_mut(), env, info, msg).unwrap();


    // Claim after registering airdrop account update
    let msg = ExecuteMsg::Claim {};

    let env = mock_env();
    let info = mock_info("terra1qfqa2eu9wp272ha93lj4yhcenrc6ymng079nu8",
                         &[Coin { denom: String::from("uusd"), amount: Uint128::from(1000000u128) }]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());

    match res {
        Err(ContractError::DoMoreTasks {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

use cosmwasm_std::{from_binary, Uint128, Timestamp, attr};
use cosmwasm_std::testing::{mock_env, mock_info};

use starterra_token::staking_gateway::{CanStakeResponse, CanStakeStatus, ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, BondAmountResponse};
use starterra_token::staking::StakerInfo;

use crate::contract::{execute, instantiate, query};
use crate::testing::mock_querier::mock_dependencies;
use crate::errors::ContractError;

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let env = mock_env();
    let info = mock_info("addr0000", &[]);
    let msg = InstantiateMsg {
        owner: String::from("owner"),
        staking_contracts: vec![String::from("addr1"), String::from("addr2")],
    };

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // it worked, let's query the state
    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        config,
        ConfigResponse {
            owner: String::from("owner"),
            staking_contracts: vec![String::from("addr1"), String::from("addr2")],
        }
    );
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies(&[]);

    let env = mock_env();
    let info = mock_info("addr0000", &[]);
    let msg = InstantiateMsg {
        owner: String::from("owner0000"),
        staking_contracts: vec![String::from("addr1")],
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // update owner
    let mut env = mock_env();
    let info = mock_info("owner0000", &[]);
    env.block.time = Timestamp::from_seconds(env.block.time.seconds() + 50);
    let pause_block_time = env.block.time.seconds();
    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("owner0001")),
        staking_contracts: None,
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = ExecuteMsg::AcceptOwnership {};
    let info = mock_info("owner0001", &vec![]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // it worked, let's query the state
    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!("owner0001", config.owner.as_str());

    // Unauthorized err
    let env = mock_env();
    let info = mock_info("owner0000", &[]);
    let msg = ExecuteMsg::UpdateConfig { owner: None, staking_contracts: None };

    let res = execute(deps.as_mut(), env.clone(), info, msg);
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }

    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        config,
        ConfigResponse {
            owner: String::from("owner0001"),
            staking_contracts: vec![String::from("addr1")],
        }
    );

    let mut env = mock_env();
    let info = mock_info("owner0001", &[]);
    env.block.time = Timestamp::from_seconds(pause_block_time + 50);
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        staking_contracts: Some(vec![String::from("addr1"), String::from("addr2")]),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        config,
        ConfigResponse {
            owner: String::from("owner0001"),
            staking_contracts: vec![String::from("addr1"), String::from("addr2")],
        }
    );
}

#[test]
fn accept_ownership() {
    let mut deps = mock_dependencies(&[]);

    let env = mock_env();
    let msg = InstantiateMsg {
        owner: String::from("owner0000"),
        staking_contracts: vec![],
    };
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
        staking_contracts: None
    };
    let info = mock_info("owner0000", &vec![]);
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
fn test_querier() {
    let mut deps = mock_dependencies(&[]);

    let env = mock_env();
    let info = mock_info("addr0000", &[]);
    let msg = InstantiateMsg {
        owner: String::from("owner0000"),
        staking_contracts: vec![String::from("staking0000"), String::from("staking0001"), String::from("staking0002")],
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // User no staking at all
    let res = query(deps.as_ref(), env.clone(), QueryMsg::CanUserStake {
        user: String::from("user0000"),
    }).unwrap();
    let can_stake: CanStakeResponse = from_binary(&res).unwrap();
    assert_eq!(
        can_stake,
        CanStakeResponse {
            statuses: vec![CanStakeStatus { staking_contract: String::from("staking0000"), can_stake: true },
                           CanStakeStatus { staking_contract: String::from("staking0001"), can_stake: true },
                           CanStakeStatus { staking_contract: String::from("staking0002"), can_stake: true }],
        }
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::BondAmount {
        user: String::from("user0000"),
    }).unwrap();
    let bond_amount: BondAmountResponse = from_binary(&res).unwrap();
    assert_eq!(
        bond_amount,
        BondAmountResponse {
            user: String::from("user0000"),
            contract: None,
            bond_amount: Uint128::zero(),
        }
    );

    // User is staking in one contract
    deps.querier.with_staker_info(vec![(
        String::from("staking0001"),
        vec![(
            String::from("user0000"),
            StakerInfo {
                reward_index: Default::default(),
                bond_amount: Uint128::from(100u64),
                pending_reward: Default::default(),
            }
        )],
    )]);

    let res = query(deps.as_ref(), env.clone(), QueryMsg::CanUserStake {
        user: String::from("user0000"),
    }).unwrap();
    let can_stake: CanStakeResponse = from_binary(&res).unwrap();
    assert_eq!(
        can_stake,
        CanStakeResponse {
            statuses: vec![CanStakeStatus { staking_contract: String::from("staking0000"), can_stake: false },
                           CanStakeStatus { staking_contract: String::from("staking0001"), can_stake: true },
                           CanStakeStatus { staking_contract: String::from("staking0002"), can_stake: false }],
        }
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::BondAmount {
        user: String::from("user0000"),
    }).unwrap();
    let bond_amount: BondAmountResponse = from_binary(&res).unwrap();
    assert_eq!(
        bond_amount,
        BondAmountResponse {
            user: String::from("user0000"),
            contract: Some(String::from("staking0001")),
            bond_amount: Uint128::from(100u64),
        }
    );

    // User is staking in multiple contracts - error
    deps.querier.with_staker_info(vec![(
        String::from("staking0001"),
        vec![(
            String::from("user0000"),
            StakerInfo {
                reward_index: Default::default(),
                bond_amount: Uint128::from(100u64),
                pending_reward: Default::default(),
            }
        )],
    ), (
        String::from("staking0002"),
        vec![(
            String::from("user0000"),
            StakerInfo {
                reward_index: Default::default(),
                bond_amount: Uint128::from(125u64),
                pending_reward: Default::default(),
            }
        )],
    ),
    ]);

    let res = query(deps.as_ref(), env.clone(), QueryMsg::CanUserStake {
        user: String::from("user0000"),
    });
    match res {
        Err(ContractError::CannotStakeInMoreThanOneContract {}) => {},
        _ => panic!("WRONG ERROR MSG"),
    }

    let res = query(deps.as_ref(), env.clone(), QueryMsg::BondAmount {
        user: String::from("user0000"),
    });
    match res {
        Err(ContractError::CannotStakeInMoreThanOneContract {}) => {},
        _ => panic!("WRONG ERROR MSG"),
    }
}

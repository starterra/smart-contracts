use crate::contract::{execute, instantiate, query};
use crate::errors::ContractError;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, from_binary};
use starterra_token::vesting_gateway::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, VestingAddressesResponse,
};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // it worked, let's query the state
    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!("owner", config.owner.as_str());
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("owner2")),
    };

    // try update config with not the owner
    let env = mock_env();
    let info = mock_info("wrong_owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }

    // update config
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = ExecuteMsg::AcceptOwnership {};
    let info = mock_info("owner2", &vec![]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // it worked, let's query the state
    let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!("owner2", config.owner.as_str());

    // Unauthorzied err
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::UpdateConfig { owner: None };

    let res = execute(deps.as_mut(), env, info, msg);
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }
}

#[test]
fn accept_ownership() {
    let mut deps = mock_dependencies(&[]);

    let env = mock_env();
    let msg = InstantiateMsg {
        owner: String::from("owner"),
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
fn register_vesting_addresses() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::UpdateVestingAddresses {
        vesting_addresses: vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
    };

    //try with not the owner
    let env = mock_env();
    let info = mock_info("wrong_owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "update_vesting_addresses"),]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::VestingAddresses {}).unwrap();
    let vesting_addresses: VestingAddressesResponse = from_binary(&res).unwrap();
    assert_eq!(
        vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
        vesting_addresses.vesting_addresses
    );
}

#[test]
fn try_register_too_many_vesting_addresses() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::UpdateVestingAddresses {
        vesting_addresses: vec![
            String::from("vestingAddr1"),
            String::from("vestingAddr2"),
            String::from("vestingAddr3"),
            String::from("vestingAddr4"),
            String::from("vestingAddr5"),
            String::from("vestingAddr6"),
            String::from("vestingAddr7"),
        ],
    };

    //try with not the owner
    let env = mock_env();
    let info = mock_info("wrong_owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg);
    match res {
        Err(ContractError::CannotHaveMoreVestingAddresses { max }) => {
            assert_eq!(max, 6)
        }
        _ => panic!("Should throw error: Too many vesting addresses (max 6)"),
    }
}

#[test]
fn add_vesting_address() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    let msg = ExecuteMsg::UpdateVestingAddresses {
        vesting_addresses: vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
    };

    //try with not the owner
    let env = mock_env();
    let info = mock_info("wrong_owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "update_vesting_addresses"),]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::VestingAddresses {}).unwrap();
    let vesting_addresses: VestingAddressesResponse = from_binary(&res).unwrap();
    assert_eq!(
        vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
        vesting_addresses.vesting_addresses
    );

    //Add vesting address
    let msg = ExecuteMsg::AddVestingAddress {
        vesting_address: String::from("vestingAddr3"),
    };

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "add_vesting_addresses"),
            attr("new_vesting_address", "vestingAddr3"),
        ]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::VestingAddresses {}).unwrap();
    let vesting_addresses: VestingAddressesResponse = from_binary(&res).unwrap();
    assert_eq!(
        vec![
            String::from("vestingAddr1"),
            String::from("vestingAddr2"),
            String::from("vestingAddr3")
        ],
        vesting_addresses.vesting_addresses
    );
}

#[test]
fn add_duplicated_vesting_address() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::UpdateVestingAddresses {
        vesting_addresses: vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "update_vesting_addresses"),]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::VestingAddresses {}).unwrap();
    let vesting_addresses: VestingAddressesResponse = from_binary(&res).unwrap();
    assert_eq!(
        vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
        vesting_addresses.vesting_addresses
    );

    //Add vesting address
    let msg = ExecuteMsg::AddVestingAddress {
        vesting_address: String::from("vestingAddr1"),
    };

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg);
    match res {
        Err(ContractError::AddressAlreadyRegistered {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

#[test]
fn try_add_seventh_vesting_address() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::UpdateVestingAddresses {
        vesting_addresses: vec![
            String::from("vestingAddr1"),
            String::from("vestingAddr2"),
            String::from("vestingAddr3"),
            String::from("vestingAddr4"),
            String::from("vestingAddr5"),
            String::from("vestingAddr6"),
        ],
    };

    let _res = execute(deps.as_mut(), env, info, msg);

    //Add vesting address
    let msg = ExecuteMsg::AddVestingAddress {
        vesting_address: String::from("vestingAddr7"),
    };

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg);
    match res {
        Err(ContractError::CannotAddMoreVestingAddresses { max }) => {
            assert_eq!(max, 6);
        }
        _ => panic!("Should throw error: CannotAddMoreVestingAddresses"),
    }
}

#[test]
fn remove_vesting_address() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::UpdateVestingAddresses {
        vesting_addresses: vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "update_vesting_addresses"),]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::VestingAddresses {}).unwrap();
    let vesting_addresses: VestingAddressesResponse = from_binary(&res).unwrap();
    assert_eq!(
        vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
        vesting_addresses.vesting_addresses
    );

    //Remove vesting address
    let msg = ExecuteMsg::RemoveVestingAddress {
        vesting_address: String::from("vestingAddr2"),
    };

    //try with not the owner
    let env = mock_env();
    let info = mock_info("wrong_owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg.clone());
    match res {
        Err(ContractError::Unauthorized { .. }) => {}
        _ => panic!("Must return unauthorized error"),
    }

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "removed_vesting_addresses"),
            attr("removed_vesting_address", "vestingAddr2"),
        ]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::VestingAddresses {}).unwrap();
    let vesting_addresses: VestingAddressesResponse = from_binary(&res).unwrap();
    assert_eq!(
        vec![String::from("vestingAddr1")],
        vesting_addresses.vesting_addresses
    );
}

#[test]
fn remove_not_existing_vesting_address() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
    };

    let env = mock_env();
    let info = mock_info("newaddr", &[]);
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::UpdateVestingAddresses {
        vesting_addresses: vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "update_vesting_addresses"),]
    );

    let res = query(deps.as_ref(), env.clone(), QueryMsg::VestingAddresses {}).unwrap();
    let vesting_addresses: VestingAddressesResponse = from_binary(&res).unwrap();
    assert_eq!(
        vec![String::from("vestingAddr1"), String::from("vestingAddr2")],
        vesting_addresses.vesting_addresses
    );

    //Remove not existing vesting address
    let msg = ExecuteMsg::RemoveVestingAddress {
        vesting_address: String::from("vestingAddr3"),
    };

    let env = mock_env();
    let info = mock_info("owner", &[]);
    let res = execute(deps.as_mut(), env, info, msg);
    match res {
        Err(ContractError::AddressNotRegistered {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

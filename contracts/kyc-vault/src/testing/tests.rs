use crate::contract::{execute, instantiate, query};
use crate::errors::ContractError;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, from_binary};
use starterra_token::kyc_vault::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, IsAcceptedResponse, IsAcceptedVerifiedResponse,
    IsVerifiedResponse, QueryMsg,
};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("newaddr", &vec![]);
    let msg = InstantiateMsg {
        owner: String::from("newaddr"),
        kyc_provider_address: String::from("kyc_provider"),
    };
    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<ConfigResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()
        )
        .unwrap(),
        ConfigResponse {
            owner: String::from("newaddr"),
            kyc_provider_address: String::from("kyc_provider"),
        }
    );
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("owner2")),
        kyc_provider_address: Some(String::from("kyc_provider2")),
    };
    let info = mock_info("owner", &vec![]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::AcceptOwnership {};
    let info = mock_info("owner2", &vec![]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<ConfigResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()
        )
        .unwrap(),
        ConfigResponse {
            owner: String::from("owner2"),
            kyc_provider_address: String::from("kyc_provider2"),
        }
    );

    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(String::from("owner3")),
        kyc_provider_address: Some(String::from("kyc_provider3")),
    };
    let info = mock_info("owner", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        kyc_provider_address: Some(String::from("kyc_provider3")),
    };
    let info = mock_info("owner2", &vec![]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<ConfigResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()
        )
        .unwrap(),
        ConfigResponse {
            owner: String::from("owner2"),
            kyc_provider_address: String::from("kyc_provider3"),
        }
    );
}

#[test]
fn accept_ownership() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
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
        kyc_provider_address: None,
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
fn register_kyc_address() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::RegisterAddress {
        address: String::from("KYC_ADDRESS1"),
    };
    let info = mock_info("wrong_kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "register_kyc_address"),
            attr("registered_kyc_address", "KYC_ADDRESS1"),
        ]
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS1"),
            is_verified: true,
        }
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS2"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS2"),
            is_verified: false,
        }
    );
}

#[test]
fn register_kyc_addresses() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::RegisterAddresses {
        addresses: vec![String::from("KYC_ADDRESS1"), String::from("KYC_ADDRESS2")],
    };
    let info = mock_info("wrong_kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "register_kyc_addresses"),
            attr("registered_kyc_addresses", "KYC_ADDRESS1,KYC_ADDRESS2"),
        ]
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS1"),
            is_verified: true,
        }
    );
    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS2"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS2"),
            is_verified: true,
        }
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS3"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS3"),
            is_verified: false,
        }
    );
}

#[test]
fn unregister_kyc_address() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::RegisterAddress {
        address: String::from("KYC_ADDRESS1"),
    };
    let info = mock_info("wrong_kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "register_kyc_address"),
            attr("registered_kyc_address", "KYC_ADDRESS1"),
        ]
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS1"),
            is_verified: true,
        }
    );

    let msg = ExecuteMsg::UnregisterAddress {
        address: String::from("KYC_ADDRESS1"),
    };

    let info = mock_info("kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "unregister_kyc_address"),
            attr("unregistered_kyc_address", "KYC_ADDRESS1"),
        ]
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS1"),
            is_verified: false,
        }
    );
}

#[test]
fn unregister_kyc_addresses() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::RegisterAddresses {
        addresses: vec![String::from("KYC_ADDRESS1"), String::from("KYC_ADDRESS2")],
    };

    let info = mock_info("wrong_kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("kyc_provider", &[]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS2"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS2"),
            is_verified: true,
        }
    );

    let msg = ExecuteMsg::UnregisterAddresses {
        addresses: vec![String::from("KYC_ADDRESS1"), String::from("KYC_ADDRESS2")],
    };

    let info = mock_info("kyc_provider", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "unregister_kyc_addresses"),
            attr("unregistered_kyc_addresses", "KYC_ADDRESS1,KYC_ADDRESS2"),
        ]
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS1"),
            is_verified: false,
        }
    );

    assert_eq!(
        from_binary::<IsVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsVerified {
                    address: String::from("KYC_ADDRESS2"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsVerifiedResponse {
            address: String::from("KYC_ADDRESS2"),
            is_verified: false,
        }
    );
}

#[test]
fn accept_terms_of_use() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::AcceptTermsOfUse {};
    let info = mock_info("user1", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "accept_terms_of_use"),
            attr("address", "user1"),
        ]
    );

    assert_eq!(
        from_binary::<IsAcceptedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsAccepted {
                    address: String::from("user1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsAcceptedResponse {
            address: String::from("user1"),
            is_accepted: true,
        }
    );

    assert_eq!(
        from_binary::<IsAcceptedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsAccepted {
                    address: String::from("user2"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsAcceptedResponse {
            address: String::from("user2"),
            is_accepted: false,
        }
    );
}

#[test]
fn is_accepted_and_verified_query() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: String::from("owner"),
        kyc_provider_address: String::from("kyc_provider"),
    };

    let info = mock_info("addr0000", &vec![]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<IsAcceptedVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsAcceptedVerified {
                    address: String::from("user1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsAcceptedVerifiedResponse {
            address: String::from("user1"),
            is_accepted: false,
            is_verified: false,
        }
    );

    let msg = ExecuteMsg::AcceptTermsOfUse {};
    let info = mock_info("user1", &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "accept_terms_of_use"),
            attr("address", "user1"),
        ]
    );

    assert_eq!(
        from_binary::<IsAcceptedVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsAcceptedVerified {
                    address: String::from("user1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsAcceptedVerifiedResponse {
            address: String::from("user1"),
            is_accepted: true,
            is_verified: false,
        }
    );

    assert_eq!(
        from_binary::<IsAcceptedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsAccepted {
                    address: String::from("user2"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsAcceptedResponse {
            address: String::from("user2"),
            is_accepted: false,
        }
    );

    //register kyc
    let msg = ExecuteMsg::RegisterAddresses {
        addresses: vec![String::from("user1")],
    };
    let info = mock_info("kyc_provider", &[]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<IsAcceptedVerifiedResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::IsAcceptedVerified {
                    address: String::from("user1"),
                },
            )
            .unwrap()
        )
        .unwrap(),
        IsAcceptedVerifiedResponse {
            address: String::from("user1"),
            is_accepted: true,
            is_verified: true,
        }
    );
}

use pretty_assertions::assert_eq;
use std::path::PathBuf;

use scilla_parser::{parse, Contract, Field, FieldList, Transition, Type};

#[test]
fn test_chain_id_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    let contract = parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "ChainId".to_string(),
            path: contract_path.canonicalize().unwrap(),
            fields: FieldList(vec![]),
            init_params: FieldList(vec![]),
            transitions: vec![Transition::new("EventChainID", FieldList::default())]
        }
    );
}

#[test]
fn test_hello_world_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/HelloWorld.scilla");
    let contract = parse(&contract_path).unwrap();

    assert_eq!(contract.path, contract_path.canonicalize().unwrap());
    assert_eq!(
        contract,
        Contract {
            name: "HelloWorld".to_string(),
            path: contract_path.canonicalize().unwrap(),
            init_params: FieldList(vec![Field::new("owner", Type::ByStr(20))]),
            fields: FieldList(vec![Field::new("welcome_msg", Type::String)]),
            transitions: vec![
                Transition::new("setHello", FieldList(vec![Field::new("msg", Type::String)])),
                Transition::new_without_param("getHello")
            ]
        }
    );
}

#[test]
fn test_send_zil_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/SendZil.scilla");
    let contract = parse(&contract_path).unwrap();

    assert_eq!(contract.path, contract_path.canonicalize().unwrap());
    assert_eq!(
        contract,
        Contract {
            name: "SendZil".to_string(),
            path: contract_path.canonicalize().unwrap(),
            init_params: FieldList(vec![]),
            fields: FieldList(vec![
                Field::new("test_field", Type::Uint256),
                Field::new("bool", Type::Bool),
                Field::new("empty_bool", Type::Option(Box::new(Type::Bool))),
                Field::new("some_int", Type::Option(Box::new(Type::Int32))),
                Field::new(
                    "pair",
                    Type::Pair(Box::new(Type::String), Box::new(Type::Uint32))
                ),
                Field::new("list", Type::List(Box::new(Type::Int32))),
            ]),
            transitions: vec![
                Transition::new_without_param("acceptZil"),
                Transition::new(
                    "updateTestField",
                    FieldList(vec![Field::new("val", Type::Uint256)])
                ),
                Transition::new_without_param("dontAcceptZil"),
                Transition::new(
                    "fundUserWithTag",
                    FieldList(vec![
                        Field::new("user", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "fundUser",
                    FieldList(vec![
                        Field::new("user", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "fundContract",
                    FieldList(vec![
                        Field::new("contract_address", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "callOtherContract",
                    FieldList(vec![
                        Field::new("contract_address", Type::ByStr(20)),
                        Field::new("tag", Type::String),
                        Field::new("value", Type::Uint256)
                    ])
                ),
            ]
        }
    );
}

#[test]
fn test_timestamp_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/Timestamp.scilla");
    let contract = parse(&contract_path).unwrap();

    assert_eq!(contract.path, contract_path.canonicalize().unwrap());
    assert_eq!(
        contract,
        Contract {
            name: "Timestamp".to_string(),
            path: contract_path.canonicalize().unwrap(),
            init_params: FieldList(vec![]),
            fields: FieldList(vec![]),
            transitions: vec![Transition::new(
                "EventTimestamp",
                FieldList(vec![Field::new("bnum", Type::BNum)])
            ),]
        }
    );
}

#[test]
fn test_fungible_token_parse() {
    let contract_path = PathBuf::from("tests/contracts/FungibleToken.scilla");
    let contract = parse(&contract_path).unwrap();
    assert_eq!(
        contract,
        Contract {
            name: "FungibleToken".to_string(),
            path: contract_path.canonicalize().unwrap(),
            init_params: FieldList(vec![
                Field::new("contract_owner", Type::ByStr(20)),
                Field::new("name", Type::String),
                Field::new("symbol", Type::String),
                Field::new("decimals", Type::Uint32),
                Field::new("init_supply", Type::Uint128)
            ]),
            fields: FieldList(vec![
                Field::new("total_supply", Type::Uint128),
                Field::new(
                    "balances",
                    Type::Map(Box::new(Type::ByStr(20)), Box::new(Type::Uint128))
                ),
                Field::new(
                    "allowances",
                    Type::Map(
                        Box::new(Type::ByStr(20)),
                        Box::new(Type::Map(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::Uint128)
                        ))
                    )
                )
            ]),
            transitions: vec![
                Transition::new(
                    "IncreaseAllowance",
                    FieldList(vec![
                        Field::new("spender", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "DecreaseAllowance",
                    FieldList(vec![
                        Field::new("spender", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "Transfer",
                    FieldList(vec![
                        Field::new("to", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "TransferFailed",
                    FieldList(vec![
                        Field::new("to", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "TransferFrom",
                    FieldList(vec![
                        Field::new("from", Type::ByStr(20)),
                        Field::new("to", Type::ByStr(20)),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
            ]
        }
    );
}

#[test]
fn test_staking_proxy_v2_parse() {
    let contract_path = PathBuf::from("tests/contracts/staking_proxy_v2.scilla");
    let contract = parse(&contract_path).unwrap();
    assert_eq!(
        contract,
        Contract {
            name: "SSNListProxy_V2".to_string(),
            path: contract_path.canonicalize().unwrap(),
            init_params: FieldList(vec![
                Field::new("init_implementation", Type::ByStr(20)),
                Field::new("init_admin", Type::ByStr(20)),
            ]),
            fields: FieldList(vec![
                Field::new("implementation", Type::ByStr(20)),
                Field::new("admin", Type::ByStr(20)),
                Field::new("stagingadmin", Type::Option(Box::new(Type::ByStr(20)))),
            ]),
            transitions: vec![
                Transition::new(
                    "UpgradeTo",
                    FieldList(vec![Field::new("newImplementation", Type::ByStr(20))])
                ),
                Transition::new(
                    "ChangeProxyAdmin",
                    FieldList(vec![Field::new("newAdmin", Type::ByStr(20))])
                ),
                Transition::new("ClaimProxyAdmin", FieldList::default()),
                Transition::new(
                    "OptInSSNToConsensusPoolAdminOverride",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr(20))])
                ),
                Transition::new(
                    "OptOutSSNFromConsensusPoolAdminOverride",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr(20))])
                ),
                Transition::new(
                    "RemoveFromConsensusPoolAdminOverride",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr(20))])
                ),
                Transition::new(
                    "ChangeMinCommissionRate",
                    FieldList(vec![Field::new("mincommrate_value", Type::Uint128)])
                ),
                Transition::new(
                    "AddSSNNonStaking",
                    FieldList(vec![
                        Field::new("ssnaddr", Type::ByStr(20)),
                        Field::new("name", Type::String),
                        Field::new("urlraw", Type::String),
                        Field::new("urlapi", Type::String),
                        Field::new("comm", Type::Uint128)
                    ])
                ),
                Transition::new("AddSSNToConsensusPool", FieldList::default()),
                Transition::new("RemoveSSNFromConsensusPool", FieldList::default()),
                Transition::new(
                    "WithdrawStakeRewardsForCycles",
                    FieldList(vec![
                        Field::new("ssnaddr", Type::ByStr(20)),
                        Field::new("cycles", Type::Uint32)
                    ])
                ),
                Transition::new(
                    "CopySSNDelegAmt",
                    FieldList(vec![
                        Field::new("ssn", Type::ByStr(20)),
                        Field::new(
                            "keys",
                            Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::Uint128)
                            )))
                        )
                    ])
                ),
                Transition::new(
                    "MigrateStakeSSNPerCycle",
                    FieldList(vec![
                        Field::new("ssn", Type::ByStr(20)),
                        Field::new(
                            "keys",
                            Type::List(Box::new(Type::Pair(
                                Box::new(Type::Uint32),
                                Box::new(Type::Pair(
                                    Box::new(Type::Uint128),
                                    Box::new(Type::Uint128)
                                ))
                            )))
                        )
                    ])
                ),
                Transition::new(
                    "CopyBuffDepositDeleg",
                    FieldList(vec![
                        Field::new("deleg", Type::ByStr(20)),
                        Field::new(
                            "keys",
                            Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                ))))
                            )))
                        )
                    ])
                ),
                Transition::new(
                    "CopyLastBufDepositCycleDelegList",
                    FieldList(vec![Field::new(
                        "last_buf_deposit_cycle_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::Uint32),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyLastWithdrawCycleDelegList",
                    FieldList(vec![Field::new(
                        "last_withdraw_cycle_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::Uint32),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDelegStakePerCycleList",
                    FieldList(vec![Field::new(
                        "deleg_stake_per_cycle_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                )))),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDirectDepositDelegList",
                    FieldList(vec![Field::new(
                        "direct_deposit_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                )))),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyBuffDepositDelegList",
                    FieldList(vec![Field::new(
                        "buff_deposit_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                )))),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDepositAmtDelegList",
                    FieldList(vec![Field::new(
                        "deposit_amt_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr(20)),
                                Box::new(Type::Uint128)
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyWithDrawalPendingList",
                    FieldList(vec![Field::new(
                        "withdrawal_pending_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::BNum),
                                Box::new(Type::Uint128)
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyCommForSSNList",
                    FieldList(vec![Field::new(
                        "comm_for_ssn_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::Uint32),
                                Box::new(Type::Uint128)
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDelegSwapRequest",
                    FieldList(vec![Field::new(
                        "deleg_swap_request_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr(20)),
                            Box::new(Type::ByStr(20)),
                        )))
                    )])
                ),
                Transition::new(
                    "ChangeCycleRewardsDeleg",
                    FieldList(vec![Field::new("input_cycle_rewards_deleg", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeVerifierReward",
                    FieldList(vec![Field::new("input_verifier_reward", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeAvailableWithdrawal",
                    FieldList(vec![Field::new(
                        "input_available_withdrawal",
                        Type::Uint128
                    )])
                ),
                Transition::new(
                    "ChangeCurrentDeleg",
                    FieldList(vec![Field::new("input_current_deleg", Type::ByStr(20))])
                ),
                Transition::new(
                    "ChangeCurrentSSN",
                    FieldList(vec![Field::new("input_current_ssn", Type::ByStr(20))])
                ),
                Transition::new(
                    "ChangeNewDeleg",
                    FieldList(vec![Field::new("input_new_deleg", Type::ByStr(20))])
                ),
                Transition::new(
                    "ChangeVerifier",
                    FieldList(vec![Field::new("input_verifier", Type::ByStr(20))])
                ),
                Transition::new(
                    "ChangeVerifierReceivingAddr",
                    FieldList(vec![Field::new(
                        "input_verifier_receiving_addr",
                        Type::ByStr(20)
                    )])
                ),
                Transition::new(
                    "ChangeMinStake",
                    FieldList(vec![Field::new("input_minstake", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeMinDelegStake",
                    FieldList(vec![Field::new("input_mindelegstake", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeLastRewardCycle",
                    FieldList(vec![Field::new("input_lastrewardcycle", Type::Uint32)])
                ),
                Transition::new(
                    "ChangeMaxCommChangeRate",
                    FieldList(vec![Field::new("input_maxcommchangerate", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeMaxCommRate",
                    FieldList(vec![Field::new("input_maxcommrate", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeTotalStakeAmount",
                    FieldList(vec![Field::new("input_totalstakeamount", Type::Uint128)])
                ),
            ]
        }
    );
}

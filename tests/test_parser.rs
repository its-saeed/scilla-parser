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

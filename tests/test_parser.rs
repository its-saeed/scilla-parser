use pretty_assertions::assert_eq;
use std::path::PathBuf;

use scilla2rust::{
    parser::{parse_sexp, Contract, Field, FieldList, Transition},
    run_scilla_fmt,
};

#[test]
fn test_chain_id_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    let sexp = run_scilla_fmt(&contract_path).unwrap();
    let contract = parse_sexp(&sexp, &contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "ChainId".to_string(),
            path: contract_path.canonicalize().unwrap(),
            fields: FieldList(vec![]),
            constructor_params: FieldList(vec![]),
            transitions: vec![Transition::new("EventChainID", FieldList::default())]
        }
    );
    assert_eq!(contract.fields.len(), 0);
    assert_eq!(contract.constructor_params.len(), 0);
    assert_eq!(contract.transitions.len(), 1);
    let transition = &contract.transitions[0];
    assert_eq!(transition.name, "EventChainID");
}

#[test]
fn test_hello_world_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/HelloWorld.scilla");
    let sexp = run_scilla_fmt(&contract_path).unwrap();
    let contract = parse_sexp(&sexp, &contract_path).unwrap();

    assert_eq!(contract.path, contract_path.canonicalize().unwrap());
    assert_eq!(
        contract,
        Contract {
            name: "HelloWorld".to_string(),
            path: contract_path.canonicalize().unwrap(),
            constructor_params: FieldList(vec![Field::new("owner", "ByStr20")]),
            fields: FieldList(vec![Field::new("welcome_msg", "String")]),
            transitions: vec![
                Transition::new("setHello", FieldList(vec![Field::new("msg", "String")])),
                Transition::new_without_param("getHello".to_string())
            ]
        }
    );
}

#[test]
fn test_send_zil_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/SendZil.scilla");
    let sexp = run_scilla_fmt(&contract_path).unwrap();
    let contract = parse_sexp(&sexp, &contract_path).unwrap();

    assert_eq!(contract.path, contract_path.canonicalize().unwrap());
    assert_eq!(
        contract,
        Contract {
            name: "SendZil".to_string(),
            path: contract_path.canonicalize().unwrap(),
            constructor_params: FieldList(vec![]),
            fields: FieldList(vec![Field::new("test_field", "Uint256")]),
            transitions: vec![
                Transition::new_without_param("acceptZil".to_string()),
                Transition::new(
                    "updateTestField",
                    FieldList(vec![Field::new("val", "Uint256",)])
                ),
                Transition::new_without_param("dontAcceptZil".to_string()),
                Transition::new(
                    "fundUserWithTag",
                    FieldList(vec![
                        Field::new("user", "ByStr20"),
                        Field::new("amount", "Uint128")
                    ])
                ),
                Transition::new(
                    "fundUser",
                    FieldList(vec![
                        Field::new("user", "ByStr20"),
                        Field::new("amount", "Uint128")
                    ])
                ),
                Transition::new(
                    "fundContract",
                    FieldList(vec![
                        Field::new("contract_address", "ByStr20"),
                        Field::new("amount", "Uint128")
                    ])
                ),
                Transition::new(
                    "callOtherContract",
                    FieldList(vec![
                        Field::new("contract_address", "ByStr20"),
                        Field::new("tag", "String"),
                        Field::new("value", "Uint256")
                    ])
                ),
            ]
        }
    );
}

#[test]
fn test_timestamp_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/Timestamp.scilla");
    let sexp = run_scilla_fmt(&contract_path).unwrap();
    let contract = parse_sexp(&sexp, &contract_path).unwrap();

    assert_eq!(contract.path, contract_path.canonicalize().unwrap());
    assert_eq!(
        contract,
        Contract {
            name: "Timestamp".to_string(),
            path: contract_path.canonicalize().unwrap(),
            constructor_params: FieldList(vec![]),
            fields: FieldList(vec![]),
            transitions: vec![Transition::new(
                "EventTimestamp",
                FieldList(vec![Field::new("bnum", "BNum")])
            ),]
        }
    );
}

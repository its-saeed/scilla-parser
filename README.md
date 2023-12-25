# Scilla Parser
This repository contains a Rust parser for the Scilla smart contract language. [Scilla](https://scilla-lang.org/) is the smart contract language used in the Zilliqa blockchain.

# Install
Add the following to your Cargo.toml:
```toml
[dependencies]
scilla_parser = "0.10.0"
```

Alternatively, You can run this command:
```shell
cargo add scilla_parser
```
This will add the scilla_parser dependency to Cargo.toml as specified in the installation instructions above.

# Usage
This library parses the s-expression of a contract. There are two options:
1. Use `Contract::from_path` and pass a contract path. This function will automatically call `scilla-fmt` through docker to generate the s-expression needed to parse the contract.
2. Parse a string (slice) to a contract. The string is supposed to have the s-expression of a contract.

## To parse a Scilla file:
Here is the code to parse [SendZil.scilla](./tests/contracts/SendZil.scilla) contract:

```rust
    use std::{error::Error, path::PathBuf};
    use scilla_parser::{Contract, Field, FieldList, Transition, TransitionList, Type};

    let contract_path = PathBuf::from("tests/contracts/SendZil.scilla");
    let contract = Contract::from_path(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "SendZil".to_string(),
            init_params: FieldList::default(),
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
            transitions: TransitionList(vec![
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
            ])
        }
    );
```

## To parse a string containing the s-expression of a scilla contract:
```rust
    let sexp: &str = "s-expression of the contract";
    let contract: Contract = sexp.parse().unwrap();
```

For more examples, take a look at the [tests](./tests/test_parser.rs).
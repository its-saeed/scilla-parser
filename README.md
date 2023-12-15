# Scilla Parser
This repository contains a Rust parser for the Scilla smart contract language. [Scilla](https://scilla-lang.org/) is the smart contract language used in the Zilliqa blockchain.

# Install
Add the following to your Cargo.toml:
```toml
[dependencies]
scilla_parser = "0.3.0"
```
Alternatively, You can run this command:
```shell
cargo add scilla_parser
```
This will add the scilla_parser dependency to Cargo.toml as specified in the installation instructions above.

# Usage
To parse a Scilla file:
```rust
    use scilla_parser::{parse, Contract, Field, FieldList, Transition};

    let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    let contract = parse(&contract_path).unwrap();

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
```
For more example take a look at the [tests](./tests/test_parser.rs).
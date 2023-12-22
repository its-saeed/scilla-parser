# Scilla Parser
This repository contains a Rust parser for the Scilla smart contract language. [Scilla](https://scilla-lang.org/) is the smart contract language used in the Zilliqa blockchain.

# Install
Add the following to your Cargo.toml:
```toml
[dependencies]
scilla_parser = "0.8.0"
```

Alternatively, You can run this command:
```shell
cargo add scilla_parser
```
This will add the scilla_parser dependency to Cargo.toml as specified in the installation instructions above.

# Usage
## To parse a Scilla file:
```rust
    use std::{error::Error, path::PathBuf};
    use scilla_parser::{Contract, Field, FieldList, Transition, TransitionList, Type};

    let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    let contract = Contract::from_path(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "ChainId".to_string(),
            fields: FieldList::default(),
            init_params: FieldList::default(),
            transitions: TransitionList(vec![Transition::new(
                "EventChainID",
                FieldList::default()
            )])
        }
    );
```

## To parse a string containing a scilla contract:
```rust
    let scilla_code: &str = "<A SCILLA CONTRACT>";
    let contract: Contract = scilla_code.parse().unwrap();
```

For more examples, take a look at the [tests](./tests/test_parser.rs).
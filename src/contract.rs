use std::{path::Path, str::FromStr};

use crate::{run_scilla_fmt, Error, FieldList, TransitionList};

#[derive(Debug, PartialEq)]
/// The `Contract` struct represents a parsed contract in Rust, including its name, initialization
/// parameters, fields, and transitions.
pub struct Contract {
    /// Name of the parsed contract
    pub name: String,
    /// List of parameters needed to deploy the contract.
    pub init_params: FieldList,
    /// List of the contract's fields.
    pub fields: FieldList,
    /// List of the contract's transitions.
    pub transitions: TransitionList,
}

impl FromStr for Contract {
    type Err = Error;

    /// Parse a Contract from a string slice
    /// # Example
    /// ```
    /// use std::{error::Error, path::PathBuf};
    /// use scilla_parser::{run_scilla_fmt, Contract, Field, FieldList, Transition, TransitionList, Type};
    /// let contract = run_scilla_fmt(&PathBuf::from("tests/contracts/chainid.scilla")).unwrap();
    ///
    /// let contract = contract.parse::<Contract>().unwrap();
    /// assert_eq!(
    ///     contract,
    ///     Contract {
    ///         name: "ChainId".to_string(),
    ///         fields: FieldList::default(),
    ///         init_params: FieldList::default(),
    ///         transitions: TransitionList(vec![Transition::new(
    ///             "EventChainID",
    ///             FieldList::default()
    ///         )])
    ///     }
    /// );
    /// ```
    fn from_str(sexp: &str) -> Result<Self, Self::Err> {
        // Bug in lexpr crate requires escaping backslashes
        let v = lexpr::from_str(&sexp.replace("\\", ""))?;
        let name = v["contr"][0]["cname"]["Ident"][0][1].to_string();
        let transitions = (&v["contr"][0]["ccomps"][0]).try_into()?;
        let init_params = (&v["contr"][0]["cparams"][0]).try_into()?;
        let fields = (&v["contr"][0]["cfields"][0]).try_into()?;
        Ok(Contract {
            name,
            transitions,
            init_params,
            fields,
        })
    }
}

impl Contract {
    /// Parse a contract from a given path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{error::Error, path::PathBuf};
    /// use scilla_parser::{Contract, Field, FieldList, Transition, TransitionList, Type};
    /// let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    /// let contract = Contract::from_path(&contract_path).unwrap();
    /// assert_eq!(
    ///     contract,
    ///     Contract {
    ///         name: "ChainId".to_string(),
    ///         fields: FieldList::default(),
    ///         init_params: FieldList::default(),
    ///         transitions: TransitionList(vec![Transition::new(
    ///             "EventChainID",
    ///             FieldList::default()
    ///         )])
    ///     }
    /// );
    /// ```
    pub fn from_path(contract_path: &Path) -> Result<Self, Error> {
        run_scilla_fmt(contract_path)?.parse()
    }
}

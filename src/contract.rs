use std::{path::Path, str::FromStr};

use crate::{run_scilla_fmt, Error, FieldList, TransitionList};

#[derive(Debug, PartialEq)]
pub struct Contract {
    pub name: String,
    pub init_params: FieldList,
    pub fields: FieldList,
    pub transitions: TransitionList,
}

impl FromStr for Contract {
    type Err = Error;

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
    pub fn from_path(contract_path: &Path) -> Result<Self, Error> {
        run_scilla_fmt(contract_path)?.parse()
    }
}

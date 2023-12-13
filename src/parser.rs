pub mod contract;
pub mod field;
pub mod transition;
pub mod r#type;

use std::path::{Path, PathBuf};

pub use contract::*;
pub use field::*;
use lexpr::Value;
pub use r#type::*;
pub use transition::*;

use crate::Error;

pub fn parse_sexp(sexp_path: &Path, contract_path: PathBuf) -> Result<Contract, Error> {
    let sexp = std::fs::read_to_string(sexp_path)?;
    let v = lexpr::from_str(&sexp)?;
    let name = v["contr"][0]["cname"]["Ident"][0][1].to_string();
    let transitions = extract_transitions(&v["contr"][0]["ccomps"])?;
    let contract_params = parse_fields(&v["contr"][0]["cparams"][0])?;
    let contract_fields = extract_contract_fields(&v["contr"][0]["cfields"])?;
    Ok(Contract {
        path: contract_path.canonicalize()?,
        name,
        transitions,
        contract_params,
        contract_fields,
    })
}

fn extract_contract_fields(cfields: &Value) -> Result<FieldList, Error> {
    let mut fields = vec![];
    for elem in cfields[0].list_iter().unwrap() {
        let field_name = elem[0]["SimpleLocal"][0].to_string();
        let field_type = elem[1][0].to_string();
        let field_type = match field_type.as_str() {
            "PrimType" => elem[1][1].to_string(),
            _ => elem[1].to_string(),
        };

        let r#type = field_type.parse()?;
        fields.push(Field {
            name: field_name,
            r#type,
        })
    }
    Ok(FieldList(fields))
}

fn extract_transitions(ccomps: &Value) -> Result<Vec<Transition>, Error> {
    let mut transitions = vec![];
    for elem in ccomps[0].list_iter().unwrap() {
        let transition_name = elem["comp_name"][0]["SimpleLocal"][0].to_string();
        transitions.push(Transition {
            name: transition_name,
            params: parse_fields(&elem["comp_params"][0])?,
        })
    }

    Ok(transitions)
}

fn parse_fields(cparams: &Value) -> Result<FieldList, Error> {
    let mut params = vec![];
    for elem in cparams.list_iter().unwrap() {
        let name = elem[0]["SimpleLocal"][0].to_string();
        let r#type = elem[1][1].to_string().parse()?;
        params.push(Field { name, r#type })
    }

    Ok(FieldList(params))
}

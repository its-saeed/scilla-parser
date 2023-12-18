use std::path::Path;

use lexpr::Value;

use crate::{run_scilla_fmt, Contract, Error, Field, FieldList, Transition};

pub fn parse(contract_path: &Path) -> Result<Contract, Error> {
    let sexp = run_scilla_fmt(contract_path).unwrap();
    let sexp = sexp.replace("\\0", "");
    parse_sexp(&sexp, contract_path)
}

pub fn parse_sexp(sexp: &str, contract_path: &Path) -> Result<Contract, Error> {
    let v = lexpr::from_str(sexp)?;
    let name = v["contr"][0]["cname"]["Ident"][0][1].to_string();
    let transitions = extract_transitions(&v["contr"][0]["ccomps"])?;
    let init_params = parse_fields(&v["contr"][0]["cparams"][0])?;
    let fields = parse_fields(&v["contr"][0]["cfields"][0])?;
    Ok(Contract {
        path: contract_path.canonicalize()?,
        name,
        transitions,
        init_params,
        fields,
    })
}

fn extract_transitions(ccomps: &Value) -> Result<Vec<Transition>, Error> {
    let mut transitions = vec![];
    for elem in ccomps[0].list_iter().unwrap() {
        let comp_type = elem["comp_type"][0].as_symbol().unwrap();
        if comp_type == "CompTrans" {
            let transition_name = elem["comp_name"][0]["SimpleLocal"][0].to_string();
            transitions.push(Transition {
                name: transition_name,
                params: parse_fields(&elem["comp_params"][0])?,
            })
        }
    }

    Ok(transitions)
}

fn parse_fields(cparams: &Value) -> Result<FieldList, Error> {
    if !cparams.is_list() {
        return Ok(FieldList::default());
    }

    let fields: Result<Vec<Field>, Error> = cparams
        .list_iter()
        .unwrap()
        .map(|elem| elem.try_into())
        .collect();

    Ok(FieldList(fields?))
}

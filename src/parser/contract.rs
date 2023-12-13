use std::path::PathBuf;

use convert_case::Casing;

use super::{FieldList, Transition};

#[derive(Debug, PartialEq)]
pub struct Contract {
    pub path: PathBuf,
    pub name: String,
    pub constructor_params: FieldList,
    pub fields: FieldList,
    pub transitions: Vec<Transition>,
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contract_name = &self.name;
        let contract_params = self
            .constructor_params
            .to_string_for_rust_function_signature();
        let contract_params_init = self.constructor_params.to_string_for_scilla_init();
        let contract_fields = self
            .fields
            .to_string_for_contract_field_getters(&format!("{contract_name}State"));
        let contract_fields_for_state_struct = self.fields.to_string_for_contract_state_struct();
        let transitions = self
            .transitions
            .iter()
            .fold("".to_string(), |acc, e| format!("{acc}{e}"));
        let transitions_as_fields = self
            .transitions
            .iter()
            .map(|tr| {
                format!(
                    "{}: RefCell<TransitionCall<T>>,",
                    tr.name.to_case(convert_case::Case::Snake)
                )
            })
            .reduce(|acc, e| format!("{acc}\n    {e}"))
            .unwrap_or_default();

        let transitions_as_fields_constructor = self
            .transitions
            .iter()
            .map(|tr| {
                format!(
                    "{}: RefCell::new(TransitionCall::new(\"{}\", &base.address, base.client.clone())),",
                    tr.name.to_case(convert_case::Case::Snake),
                    tr.name
                )
            })
            .reduce(|acc, e| format!("{acc}\n            {e}"))
            .unwrap_or_default();

        write!(
            f,
            r#"#[derive(Debug)]
pub struct {contract_name}<T: Middleware> {{
    pub base: BaseContract<T>,
    {transitions_as_fields}
}}

impl<T: Middleware> {contract_name}<T> {{
    pub async fn deploy(client: Arc<T> {contract_params}) -> Result<Self, Error> {{
        let factory = ContractFactory::new(client.clone());
        let init = Init(vec![
            Value::new("_scilla_version".to_string(), "Uint32".to_string(), "0".to_string()),
            {contract_params_init}
        ]);

        Ok(Self::new(factory.deploy_from_file(&std::path::PathBuf::from("{}"), init, None).await?))
    }}

    pub fn address(&self) -> &ZilAddress  {{
        &self.base.address
    }}

    pub fn new(base: BaseContract<T>) -> Self {{
        Self{{
            {transitions_as_fields_constructor}
            base,
        }}
    }}
    {transitions}{contract_fields}
    pub async fn get_state(&self) -> Result<{contract_name}State, Error> {{
        self.base.get_state().await
    }}
}}

#[derive(serde::Deserialize, Debug)]
pub struct {contract_name}State {{{contract_fields_for_state_struct}
}}
"#,
            self.path.display()
        )
    }
}

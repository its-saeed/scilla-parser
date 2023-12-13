use std::fmt::Display;

use convert_case::Casing;

use super::Type;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug)]
pub struct FieldList(pub Vec<Field>);

impl FieldList {
    pub fn to_string_for_rust_function_signature(&self) -> String {
        self.iter()
            .fold("".to_string(), |acc, e| format!("{acc}, {e}"))
    }

    pub fn to_string_for_contract_field_getters(&self, state_struct_name: &str) -> String {
        self.iter()
            .map(|field| {
                format!(
                    "    pub async fn {}(&self) -> Result<{}, Error> {{\n        Ok(self.base.get_state::<{state_struct_name}>().await?.{})\n    }}",
                    field.name, field.r#type.rust_type, field.name
                )
            })
            .fold("".to_string(), |acc, e| format!("{acc}\n{e}"))
    }

    pub fn to_string_for_contract_state_struct(&self) -> String {
        self.iter()
            .map(|field| format!("    pub {}: {},", field.name, field.r#type.rust_type))
            .fold("".to_string(), |acc, e| format!("{acc}\n{e}"))
    }

    pub fn to_string_for_scilla_init(&self) -> String {
        self.iter().fold("".to_string(), |acc, e| {
            let delim = if acc.is_empty() { "" } else { ", " };
            format!(
                r#"{acc}{delim}Value::new("{}".to_string(), "{}".to_string(), {}) "#,
                e.name,
                e.r#type.scilla_type,
                e.name.to_case(convert_case::Case::Snake)
            )
        })
    }
}

impl std::ops::Deref for FieldList {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name.to_case(convert_case::Case::Snake),
            self.r#type.rust_type
        )
    }
}

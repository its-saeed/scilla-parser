use convert_case::Casing;

use super::FieldList;

#[derive(Debug, PartialEq)]
pub struct Transition {
    pub name: String,
    pub params: FieldList,
}

impl Transition {
    pub fn new(name: &str, params: FieldList) -> Self {
        Self {
            name: name.to_string(),
            params,
        }
    }

    pub fn new_without_param(name: String) -> Self {
        Self {
            name,
            params: FieldList::default(),
        }
    }
}

impl std::fmt::Display for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let transition_name_snake = self.name.to_case(convert_case::Case::Snake);
        write!(
            f,
            r#"
    pub fn {transition_name_snake}(&self {}) -> RefMut<'_, transition_call::TransitionCall<T>> {{
        self.{transition_name_snake}.borrow_mut().args(vec![{}]);
        self.{transition_name_snake}.borrow_mut()
    }}
"#,
            self.params.to_string_for_rust_function_signature(),
            self.params.to_string_for_scilla_init()
        )
    }
}

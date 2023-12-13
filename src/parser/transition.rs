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

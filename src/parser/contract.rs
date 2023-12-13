use std::path::PathBuf;

use super::{FieldList, Transition};

#[derive(Debug, PartialEq)]
pub struct Contract {
    pub path: PathBuf,
    pub name: String,
    pub constructor_params: FieldList,
    pub fields: FieldList,
    pub transitions: Vec<Transition>,
}

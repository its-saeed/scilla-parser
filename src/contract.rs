use std::path::PathBuf;

use crate::{FieldList, Transition};

#[derive(Debug, PartialEq)]
pub struct Contract {
    pub path: PathBuf,
    pub name: String,
    pub init_params: FieldList,
    pub fields: FieldList,
    pub transitions: Vec<Transition>,
}

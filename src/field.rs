use lexpr::Value;

use crate::{Error, Type};

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub r#type: Type,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            name: Default::default(),
            r#type: Type::Other(Default::default()),
        }
    }
}

impl Field {
    pub fn new(name: &str, r#type: Type) -> Self {
        Self {
            name: name.to_string(),
            r#type,
        }
    }
}

impl TryFrom<&Value> for Field {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let name = value[0]["SimpleLocal"][0].to_string();
        let r#type = value[1].to_string().parse()?;

        Ok(Field { name, r#type })
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct FieldList(pub Vec<Field>);

impl std::ops::Deref for FieldList {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

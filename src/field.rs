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
    /// Creates a new instance of field.
    ///
    /// Arguments:
    ///
    /// * `name`: A string representing the name of the field.
    /// * `r#type`: Type of the field.
    pub fn new(name: &str, r#type: Type) -> Self {
        Self {
            name: name.to_string(),
            r#type,
        }
    }
}

impl TryFrom<&Value> for Field {
    type Error = Error;

    /// Try to parse a field from a lexpr::Value
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

impl TryFrom<&Value> for FieldList {
    type Error = Error;

    /// Try to parse a list of fields from a lexpr::Value
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if !value.is_list() {
            return Ok(FieldList::default());
        }

        let fields: Result<Vec<Field>, Error> = value
            .list_iter()
            .unwrap()
            .map(|elem| elem.try_into())
            .collect();

        Ok(FieldList(fields?))
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Field {
    pub name: String,
    pub r#type: String,
}

impl Field {
    pub fn new(name: &str, r#type: &str) -> Self {
        Self {
            name: name.to_string(),
            r#type: r#type.to_string(),
        }
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

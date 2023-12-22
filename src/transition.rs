use lexpr::Value;

use crate::{Error, FieldList};

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

    pub fn new_without_param(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: FieldList::default(),
        }
    }
}

impl TryFrom<&Value> for Transition {
    type Error = Error;

    /// Try to parse a lexpr::Value into a transition.
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let comp_type = value["comp_type"][0].as_symbol().unwrap();
        if comp_type == "CompTrans" {
            let transition_name = value["comp_name"][0]["SimpleLocal"][0].to_string();
            Ok(Transition {
                name: transition_name,
                params: (&value["comp_params"][0]).try_into()?,
            })
        } else {
            Err(Error::CompTypeIsNotTransition(comp_type.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct TransitionList(pub Vec<Transition>);

impl std::ops::Deref for TransitionList {
    type Target = Vec<Transition>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&Value> for TransitionList {
    type Error = Error;

    /// Try to parse a lexpr::Value into a list of transitions.
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if !value.is_list() {
            return Ok(TransitionList::default());
        }

        Ok(TransitionList(
            value
                .list_iter()
                .unwrap()
                .filter_map(|elem| Transition::try_from(elem).ok())
                .collect(),
        ))
    }
}

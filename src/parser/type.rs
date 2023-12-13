use std::str::FromStr;

use crate::Error;

#[derive(Debug, PartialEq, Default)]
pub struct Type {
    pub scilla_type: String,
    pub rust_type: String,
}

impl Type {
    pub fn new(scilla_type: String, rust_type: String) -> Self {
        Self {
            scilla_type,
            rust_type,
        }
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let rust_type = match s {
            "Int64" => "i64",
            "Int128" => "i128",
            "Int256" => "i256",
            "Uint32" => "u32",
            "Uint64" => "u64",
            "Uint128" => "u128",
            "BNum" | "Uint256" => "primitive_types::U256",
            "ByStr20" | "String" => "String",
            _ => return Err(Error::FailedToMapScillaTypeToRust(s.to_string())),
        };

        Ok(Self {
            rust_type: rust_type.to_string(),
            scilla_type: s.to_string(),
        })
    }
}

use std::str::FromStr;

use crate::Error;

#[derive(Debug, PartialEq)]
pub enum Type {
    Int32,
    Int64,
    Int128,
    Int256,

    Uint32,
    Uint64,
    Uint128,
    Uint256,

    String,

    BNum,
    Map(Box<Type>, Box<Type>),

    ByStr(usize),

    Other(String),
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = lexpr::from_str(s)?;
        match v[0].as_symbol() {
            Some(t) => match t {
                "PrimType" => match v[1].as_symbol().unwrap() {
                    "Int32" => Ok(Type::Int32),
                    "Int64" => Ok(Type::Int64),
                    "Int128" => Ok(Type::Int128),
                    "Int256" => Ok(Type::Int256),
                    "Uint32" => Ok(Type::Uint32),
                    "Uint64" => Ok(Type::Uint64),
                    "Uint128" => Ok(Type::Uint128),
                    "Uint256" => Ok(Type::Uint256),
                    "String" => Ok(Type::String),
                    "ByStr20" => Ok(Type::ByStr(20)),
                    "BNum" => Ok(Type::BNum),
                    _ => Ok(Type::Other(s.to_string())),
                },
                "MapType" => Ok(Type::Map(
                    Box::new(v[1].to_string().parse()?),
                    Box::new(v[2].to_string().parse()?),
                )),
                _ => Ok(Type::Other(s.to_string())),
            },
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prim_type() {
        let prim = "(PrimType String)".parse::<Type>().unwrap();
        assert_eq!(prim, Type::String);
    }

    #[test]
    fn test_map_type() {
        let map_type = "(MapType (PrimType ByStr20) (PrimType Uint128))"
            .parse::<Type>()
            .unwrap();

        assert_eq!(
            map_type,
            Type::Map(Box::new(Type::ByStr(20)), Box::new(Type::Uint128))
        );
    }

    #[test]
    fn test_complex_map_type() {
        let map_type =
            "(MapType (PrimType ByStr20) (MapType (PrimType ByStr20) (PrimType Uint128)))"
                .parse::<Type>()
                .unwrap();

        assert_eq!(
            map_type,
            Type::Map(
                Box::new(Type::ByStr(20)),
                Box::new(Type::Map(
                    Box::new(Type::ByStr(20)),
                    Box::new(Type::Uint128)
                ))
            )
        );
    }
}

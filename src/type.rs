use std::{fmt::Display, str::FromStr};

use crate::Error;

/// Represents all different scilla types.
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

    // ADT
    Bool,
    Option(Box<Type>),
    Pair(Box<Type>, Box<Type>),
    List(Box<Type>),

    Other(String),
}

impl FromStr for Type {
    type Err = Error;

    /// Try to parse a string slice to a Type.
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
                "ADT" => match v["Ident"]["SimpleLocal"][0].as_symbol().unwrap() {
                    "Bool" => Ok(Type::Bool),
                    "Option" => Ok(Type::Option(Box::new(v[2][0].to_string().parse()?))),
                    "List" => Ok(Type::List(Box::new(v[2][0].to_string().parse()?))),
                    "Pair" => Ok(Type::Pair(
                        Box::new(v[2][0].to_string().parse()?),
                        Box::new(v[2][1].to_string().parse()?),
                    )),
                    _ => Ok(Type::Other(v["Ident"]["SimpleLocal"][0].to_string())),
                },
                _ => Ok(Type::Other(s.to_string())),
            },
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int32 => write!(f, "Int32"),
            Type::Int64 => write!(f, "Int64"),
            Type::Int128 => write!(f, "Int128"),
            Type::Int256 => write!(f, "Int256"),
            Type::Uint32 => write!(f, "Uint32"),
            Type::Uint64 => write!(f, "Uint64"),
            Type::Uint128 => write!(f, "Uint128"),
            Type::Uint256 => write!(f, "Uint256"),
            Type::String => write!(f, "String"),
            Type::BNum => write!(f, "BNum"),
            Type::Bool => write!(f, "Bool"),
            Type::Map(ref k, ref v) => write!(f, "(Map {}, {})", k, v),
            Type::Option(ref k) => write!(f, "(Option {})", k),
            Type::List(ref k) => write!(f, "(List {})", k),
            Type::Pair(ref k, ref v) => write!(f, "(Pair {} {})", k, v),
            Type::ByStr(n) => write!(f, "ByStr{}", n),
            Type::Other(ref s) => write!(f, "{}", s),
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

    #[test]
    fn test_bool_type() {
        let bool_type = "(ADT (Ident (SimpleLocal Bool) ((fname \"\") (lnum 0) (cnum 0))) ())"
            .parse::<Type>()
            .unwrap();

        assert_eq!(bool_type, Type::Bool);
    }

    #[test]
    fn test_option_bool_type() {
        let option_bool_type = r#"(ADT (Ident (SimpleLocal Option) ((fname "") (lnum 0) (cnum 0))) ((ADT (Ident (SimpleLocal Bool) ((fname "") (lnum 0) (cnum 0))) ())))"#
            .parse::<Type>()
            .unwrap();

        assert_eq!(option_bool_type, Type::Option(Box::new(Type::Bool)));
    }

    #[test]
    fn test_type_to_string() {
        //(List (Pair ByStr20 (List (Pair ByStr20 Uint32))))
        let list_type = Type::List(Box::new(Type::Pair(
            Box::new(Type::ByStr(20)),
            Box::new(Type::List(Box::new(Type::Pair(
                Box::new(Type::ByStr(20)),
                Box::new(Type::Uint32),
            )))),
        )));

        assert_eq!(
            "(List (Pair ByStr20 (List (Pair ByStr20 Uint32))))",
            list_type.to_string()
        );

        // (List (Pair ByStr20 (List (Pair ByStr20 (List (Pair Uint32 Uint128))))))
        let list_type = Type::List(Box::new(Type::Pair(
            Box::new(Type::ByStr(20)),
            Box::new(Type::List(Box::new(Type::Pair(
                Box::new(Type::ByStr(20)),
                Box::new(Type::List(Box::new(Type::Pair(
                    Box::new(Type::Uint32),
                    Box::new(Type::Uint128),
                )))),
            )))),
        )));

        assert_eq!(
            "(List (Pair ByStr20 (List (Pair ByStr20 (List (Pair Uint32 Uint128))))))",
            list_type.to_string()
        );
    }
}

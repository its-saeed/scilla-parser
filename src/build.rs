use anyhow::{anyhow, Context, Result};
use convert_case::Casing;
use lexpr::Value;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    add_to_log("Start...");
    let contracts_path = env::var("CONTRACTS_PATH")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("contracts"));

    add_to_log(&format!("Contract path: {}", contracts_path.display()));
    if let Err(x) = generate(contracts_path) {
        add_to_log(&x.to_string())
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=contracts");
    Ok(())
}

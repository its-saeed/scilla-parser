pub mod contract;
pub mod error;
pub mod field;
pub mod parser;
pub mod transition;
pub mod r#type;

pub use contract::*;
pub use error::Error;
pub use field::*;
pub use parser::*;
pub use r#type::*;
pub use transition::*;

use std::{path::Path, process::Command};

pub fn run_scilla_fmt(path: &Path) -> Result<String, Error> {
    //docker run --rm -v contract.scilla:/tmp/input.scilla  -i zilliqa/scilla:v0.13.3 /scilla/0/bin/scilla-fmt --sexp --human-readable -d /tmp/input.scilla
    let volume = &format!(
        "{}:/tmp/input.scilla",
        path.canonicalize().unwrap().display()
    );

    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            volume,
            "-i",
            "zilliqa/scilla:v0.13.3",
            "/scilla/0/bin/scilla-fmt",
            "--sexp",
            "--human-readable",
            "-d",
            "/tmp/input.scilla",
        ])
        .output()?;

    Ok(String::from_utf8(output.stdout)?)
}

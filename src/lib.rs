pub mod error;
pub mod parser;

pub use error::Error;
pub use parser::*;

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

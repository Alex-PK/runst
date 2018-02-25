#[macro_use]
extern crate failure;

use std::env;
use std::process::{self, Command, Stdio};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::{self, File};

use failure::Error;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        eprintln!("usage: runst <source file> [...]");
        process::exit(1);
    }

    let _ = run(args);
}

fn run(args: Vec<String>) -> Result<(), Error> {
    let metadata = fs::metadata(&args[0])?;
    if !metadata.file_type().is_file() {
        return Err(RuntimeError::NotRegular(args[0].to_string()).into());
    }

    let mut source = String::with_capacity(metadata.len() as usize);
    BufReader::new(File::open(&args[0])?).read_to_string(&mut source)?;

    let target = "/tmp/runstexe";

    // rustc --crate-name runst --crate-type bin --emit=link -C opt-level=3  --out-dir ./ -
    let mut cmd = Command::new("rustc")
        .stdin(Stdio::piped())
        .args(&[
            "--crate-name",
            "runstexe",
            "--crate-type",
            "bin",
            "--emit=link",
            "-C",
            "opt-level=3",
            "-o",
            target,
            "-",
        ])
        .spawn()?;

    {
        let mut writer = BufWriter::new(cmd.stdin.take().unwrap());
        writer.write_all(source.as_bytes())?;
    }


    let compiler_result = cmd.wait_with_output()?;

    if !compiler_result.status.success() {
        return Err(RuntimeError::CompilerError(
            compiler_result.status.code().unwrap_or(0),
            String::from_utf8_lossy(&compiler_result.stderr).to_string(),
        ).into());
    }

    Command::new(target)
        .args(&args[1..])
        .spawn()?
        .wait()?;

    fs::remove_file(target)?;

    Ok(())
}

#[derive(Debug, Fail)]
enum RuntimeError {
    #[fail(display = "not a regular file: {}", _0)] NotRegular(String),

    #[fail(display = "compiler failed with status {}:\n{}", _0, _1)] CompilerError(i32, String),
}

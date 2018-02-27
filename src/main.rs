/*
	Copyright 2018 Alessandro Pellizzari

	This file is part of strange.

	Runst is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, version 2.

	Runst is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with Runst.  If not, see <http://www.gnu.org/licenses/>.
*/

#[macro_use]
extern crate failure;
extern crate nix;

use std::env;
use std::process::{self, Command, Stdio};
use std::fs;
use std::time;

use failure::{Error, ResultExt};
use nix::unistd::Uid;


fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        eprintln!("usage: runst <source file> [...]");
        process::exit(1);
    }

    let res = run(args);
    match res {
        Ok(code) => {
            process::exit(code)
        },
        Err(e) => {
            for c in e.causes() {
                println!("{}", c);
            }
            println!("{}", e.backtrace());
            process::exit(126);
        }
    }
}

fn run(args: Vec<String>) -> Result<i32, Error> {
    let source = fs::canonicalize(&args[0]).context("Getting script path")?;

    let source_metadata = fs::metadata(&source).context("Retrieving script metadata")?;
    if !source_metadata.file_type().is_file() {
        return Err(RuntimeError::NotRegular(args[0].to_string()).into());
    }

    let source = source
        .as_os_str()
        .to_str()
        .unwrap();
    let uid = Uid::current();

    let target = env::temp_dir()
        .join(&format!("runst-{}{}", uid, source.replace("/", "-")));
    let target = target
        .as_os_str()
        .to_str()
        .unwrap();

    let cached = fs::metadata(target)
        .map(|d|
            d.modified().unwrap_or(time::UNIX_EPOCH) > source_metadata.modified().unwrap_or(time::UNIX_EPOCH)
        ).unwrap_or(false);

    if !cached {
        // rustc --crate-name runst --crate-type bin --emit=link -C opt-level=3  --out-dir ./ -
        let cmd = Command::new("rustc")
            .stdin(Stdio::null())
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
                source,
            ])
            .spawn()
            .context("Running rust compiler")?;

        let compiler_result = cmd.wait_with_output()?;

        if !compiler_result.status.success() {
            return Err(RuntimeError::CompilerError(
                compiler_result.status.code().unwrap_or(0),
                String::from_utf8_lossy(&compiler_result.stderr).to_string(),
            ).into());
        }
    }

    let exit_status = Command::new(&target)
        .args(&args[1..])
        .spawn().context("Launching script")?
        .wait()
        .context("Running script")?;

    Ok(exit_status.code().unwrap_or(0))
}

#[derive(Debug, Fail)]
enum RuntimeError {
    #[fail(display = "not a regular file: {}", _0)] NotRegular(String),

    #[fail(display = "compiler failed with status {}:\n{}", _0, _1)] CompilerError(i32, String),
}

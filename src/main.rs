#![deny(clippy::arithmetic_side_effects)]

use clap::Parser;
use errgonomic::exit_result;
use std::process::ExitCode;

fn main() -> ExitCode {
    let result = Command::parse().run();
    exit_result(result)
}

mod command;

pub use command::*;

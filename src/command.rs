use clap::Parser;
use errgonomic::handle;
use mkpar::create_parent_directories;
use std::process::ExitCode;
use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(author, version, about, propagate_version = true, flatten_help = true, disable_help_subcommand = true)]
pub struct Command {
    #[arg(value_name = "PATH", help = "Path whose parent directories should be created")]
    pub path: PathBuf,
}

impl Command {
    pub fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;

        let Self {
            path,
        } = self;

        handle!(create_parent_directories(&path), CreateParentDirectoriesFailed, path);

        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Debug, Error)]
pub enum CommandRunError {
    #[error("failed to create parent directories for path '{path}'")]
    CreateParentDirectoriesFailed { source: io::Error, path: PathBuf },
}

#[cfg(test)]
mod tests {
    use crate::Command;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Command::command().debug_assert();
    }
}

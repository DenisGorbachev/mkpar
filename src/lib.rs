//! `mkpar <PATH>` creates all missing parent directories for the given path.
//!
//! For example, `mkpar logs/app/output.log` creates `logs/app` without creating or modifying `output.log`.

mod create_parent_directories;

pub use create_parent_directories::*;

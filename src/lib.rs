//! This is a module-level comment for a Rust lib

#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

use clap as _;
use errgonomic as _;
use thiserror as _;

mod create_parent_directories;

pub use create_parent_directories::*;

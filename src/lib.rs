#![doc = include_str!("../docs/README.md")]

mod cli;
mod errors;
mod publish;

pub use cli::do_action;

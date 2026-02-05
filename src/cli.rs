use crate::errors::*;
use crate::{develop, publish};
use antex::{StyledText, Text, auto};
use clap::{Arg, ArgAction, ArgMatches, Command, command};
use std::path::Path;

enum Action {
  /// Publish workspace crates.
  Publish(
    /// Path to the manifest file of the workspace.
    String,
    /// Perform all checks without publishing crates when `true`.
    bool,
    /// All questions will be answered with `yes` when `true`.
    bool,
  ),
  /// Switch workspace crates to local development mode.
  Develop(
    /// Path to the manifest file of the workspace.
    String,
    /// All questions will be answered with `yes` when `true`.
    bool,
  ),
  /// Do nothing.
  Nothing,
}

/// Parses CLI argument matches.
fn get_matches() -> ArgMatches {
  command!()
    .subcommand(
      Command::new("publish")
        .about("Publish workspace crates")
        .display_order(1)
        .arg(
          Arg::new("dir")
            .short('d')
            .long("dir")
            .help("Directory with workspace manifest")
            .default_value(".")
            .num_args(1)
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(
          Arg::new("dry-run")
            .long("dry-run")
            .help("Perform all checks without publishing")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .default_missing_value("true")
            .display_order(2),
        )
        .arg(
          Arg::new("accept-all")
            .short('y')
            .long("accept-all")
            .help("Answer all questions with 'yes'")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .default_missing_value("true")
            .display_order(3),
        ),
    )
    .subcommand(
      Command::new("develop")
        .about("Switch workspace crates to local development mode")
        .display_order(2)
        .arg(
          Arg::new("dir")
            .short('d')
            .long("dir")
            .help("Directory with workspace manifest")
            .default_value(".")
            .num_args(1)
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(
          Arg::new("accept-all")
            .short('y')
            .long("accept-all")
            .help("Answer all questions with 'yes'")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .default_missing_value("true")
            .display_order(2),
        ),
    )
    .get_matches()
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to a valid argument.
fn get_cli_action() -> Action {
  let matches = get_matches();
  match matches.subcommand() {
    Some(("publish", matches)) => {
      let dir = match_string(matches, "dir");
      let dry_run = match_boolean(matches, "dry-run");
      let accept_all = match_boolean(matches, "accept-all");
      return Action::Publish(dir, dry_run, accept_all);
    }
    Some(("develop", matches)) => {
      let dir = match_string(matches, "dir");
      let accept_all = match_boolean(matches, "accept-all");
      return Action::Develop(dir, accept_all);
    }
    _ => {}
  }
  Action::Nothing
}

pub fn do_action() {
  fn error_message(reason: UniverError) -> Text {
    auto().bold().red().s("error").clear().s(": ").s(reason.to_string())
  }

  match get_cli_action() {
    Action::Publish(dir, dry_run, accept_all) => {
      // Publish workspace crates.
      match publish::publish(Path::new(&dir), dry_run, accept_all) {
        Ok(()) => {}
        Err(reason) => {
          eprintln!("{}", error_message(reason));
          std::process::exit(1);
        }
      }
    }
    Action::Develop(dir, accept_all) => {
      // Switch workspace crates to local development mode.
      match develop::develop(Path::new(&dir), accept_all) {
        Ok(()) => {}
        Err(reason) => {
          eprintln!("{}", error_message(reason));
          std::process::exit(1);
        }
      }
    }
    Action::Nothing => {
      // No action was requested.
    }
  }
}

/// Matches a mandatory string argument.
fn match_string(matches: &ArgMatches, name: &str) -> String {
  matches.get_one::<String>(name).unwrap().trim().to_string()
}

/// Matches a mandatory boolean argument.
fn match_boolean(matches: &ArgMatches, name: &str) -> bool {
  matches.get_flag(name)
}

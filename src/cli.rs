use crate::errors::*;
use antex::{StyledText, Text, auto};
use clap::{Arg, ArgAction, ArgMatches, Command, command};
use crate::publish;

/// Default timeout in seconds.
const DEFAULT_TIMEOUT: u64 = 5;

enum Action {
  Publish(
    /// Path to the workspace manifest file.
    String,
    /// Number of seconds to wait after publishing each crate.
    u64,
    /// Flag indicating if all questions should be answered with 'Y'.
    bool,
    /// Flag indicating if only simulate publishing crates.
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
        .about("Publish crates")
        .display_order(5)
        .arg(
          Arg::new("dir")
            .short('d')
            .long("dir")
            .help("Directory where the workspace manifest file is placed")
            .default_value(".")
            .num_args(1)
            .action(ArgAction::Set)
            .display_order(2),
        )
        .arg(
          Arg::new("timeout")
            .short('t')
            .long("timeout")
            .help("Number of seconds to wait after publishing each crate")
            .default_value("5")
            .num_args(1)
            .action(ArgAction::Set)
            .display_order(3),
        )
        .arg(
          Arg::new("accept-all")
            .short('y')
            .long("accept-all")
            .help("Answer all questions with 'Y'")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .default_missing_value("true")
            .display_order(4),
        )
        .arg(
          Arg::new("simulation")
            .short('s')
            .long("simulation")
            .help("Perform only a simulation, no crates will be published")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .default_missing_value("true")
            .display_order(5),
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
      let timeout = match_string(matches, "timeout").parse::<u64>().unwrap_or(DEFAULT_TIMEOUT).clamp(0, 60);
      let accept_all = match_boolean(matches, "accept-all");
      let simulation = match_boolean(matches, "simulation");
      return Action::Publish(dir, timeout, accept_all, simulation);
    }
    _ => {}
  }
  Action::Nothing
}

pub fn do_action() {
  fn error_message(reason: UniverError) -> Text {
    auto().bold().red().s("error").clear().s(": ").s(reason.to_string())
  }

  //
  match get_cli_action() {
    Action::Publish(dir, timeout, accept_all, simulation) => {
      // Publish crates.
      match publish::publish(&dir, timeout, accept_all, simulation) {
        Ok(()) => {}
        Err(reason) => {
          eprintln!("{}", error_message(reason));
          std::process::exit(1);
        }
      }
    }
    Action::Nothing => {
      // No specific action was requested.
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

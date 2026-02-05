#[test]
fn short() {
  let expected = r#"Unified versions publisher

Usage: univer [COMMAND]

Commands:
  publish  Publish workspace crates
  develop  Switch workspace crates to local development mode
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
"#;
  cli_assert::command!().arg("-h").code(0).stdout(expected).stderr("").execute();
}

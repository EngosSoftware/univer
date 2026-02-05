#[test]
fn _0001() {
  cli_assert::command!().code(1).arg("develop").stdout("").stderr("error: no crates to develop\n").execute();
}

#[test]
fn _0002() {
  cli_assert::command!().code(0).arg("develop").arg("--accept-all").stdout("").stderr("").execute();
}

#[test]
fn _0001() {
  cli_assert::command!()
    .code(1)
    .arg("publish")
    .stdout("")
    .stderr("error: no crates to publish\n")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("--simulation")
    .stdout("")
    .stderr("")
    .execute();
}

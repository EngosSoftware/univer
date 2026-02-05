use super::*;

use std::path::Path;

const EXPECTED: &str = r#"[workspace]
members = ["packages/*"]

resolver = "2"

[workspace.package]
version = "3.0.2"

[workspace.dependencies]
cosmwasm-core = { path = "packages/core" }
cosmwasm-crypto = { path = "packages/crypto" }
cosmwasm-derive = { path = "packages/derive" }
cw-schema-derive = { path = "packages/cw-schema-derive" }
cw-schema = { path = "packages/cw-schema" }
cosmwasm-schema-derive = { path = "packages/schema-derive" }
cosmwasm-schema = { path = "packages/schema" }
cosmwasm-std = { path = "packages/std", default-features = false }
cosmwasm-vm-derive = { path = "packages/vm-derive" }
cosmwasm-vm = { path = "packages/vm" }
cosmwasm-check = { path = "packages/check" }
go-gen = { path = "./packages/go-gen" }
schemars = "1.2.1"
serde = { version = "1.0.228", default-features = false, features = ["alloc", "derive"] }
serde_json = "1.0.149"
thiserror = "2.0.18"
"#;

/// This test verifies replacing versions with paths.
#[test]
fn _0001() {
  // Make a copy of the original Cargo.toml file.
  let working_dir = Path::new(file!()).parent().unwrap();
  let original = working_dir.join(Path::new("Cargo.toml"));
  let backup = working_dir.join(Path::new("Cargo.toml.bak"));
  std::fs::copy(&original, &backup).unwrap();
  // Replace version numbers with local paths.
  cli_assert::command!().code(0).arg("develop").stdout("").stderr("").execute();
  // Make sure the Cargo.toml file is modified properly.
  assert_eq!(normalize(EXPECTED), std::fs::read_to_string(&original).unwrap());
  // Revert changes to Cargo.toml file.
  std::fs::copy(&backup, original).unwrap();
  std::fs::remove_file(backup).unwrap()
}

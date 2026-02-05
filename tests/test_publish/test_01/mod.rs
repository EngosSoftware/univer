use super::*;

use std::path::Path;

const EXPECTED: &str = r#"[workspace]
members = ["packages/*"]

resolver = "2"

[workspace.package]
version = "3.0.2"

[workspace.dependencies]
cosmwasm-core = { version = "3.0.2" }
cosmwasm-crypto = { version = "3.0.2" }
cosmwasm-derive = { version = "3.0.2" }
cw-schema-derive = { version = "3.0.2" }
cw-schema = { version = "3.0.2" }
cosmwasm-schema-derive = { version = "3.0.2" }
cosmwasm-schema = { version = "3.0.2" }
cosmwasm-std = { version = "3.0.2", default-features = false }
cosmwasm-vm-derive = { version = "3.0.2" }
cosmwasm-vm = { version = "3.0.2" }
cosmwasm-check = { version = "3.0.2" }
go-gen = { path = "./packages/go-gen" }
schemars = "1.2.1"
serde = { version = "1.0.228", default-features = false, features = ["alloc", "derive"] }
serde_json = "1.0.149"
thiserror = "2.0.18"
"#;

/// This test verifies replacing paths with versions.
#[test]
fn _0001() {
  // Make a copy of the original Cargo.toml file.
  let working_dir = Path::new(file!()).parent().unwrap();
  let original = working_dir.join(Path::new("Cargo.toml"));
  let backup = working_dir.join(Path::new("Cargo.toml.bak"));
  std::fs::copy(&original, &backup).unwrap();
  // Publish workspace crates.
  cli_assert::command!().code(0).arg("publish").arg("--dry-run").stdout("").stderr("").execute();
  // Make sure the Cargo.toml file is modified properly.
  assert_eq!(normalize(EXPECTED), std::fs::read_to_string(&original).unwrap());
  // Revert changes to Cargo.toml file.
  std::fs::copy(&backup, original).unwrap();
  std::fs::remove_file(backup).unwrap()
}

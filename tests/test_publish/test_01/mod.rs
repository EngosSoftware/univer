use super::*;

use std::path::Path;

const EXPECTED_FILE: &str = r#"[workspace]
members = ["packages/*"]

resolver = "2"

[workspace.package]
version = "3.0.2"

[workspace.dependencies]
cosmwasm-core = { version = "3.0.2" }
cosmwasm-crypto = { version = "3.0.2" }
cosmwasm-derive = { version = "3.0.2" }
cosmwasm-schema = { version = "3.0.2" }
cosmwasm-schema-derive = { version = "3.0.2" }
cosmwasm-std = { version = "3.0.2", default-features = false }
cosmwasm-vm = { version = "3.0.2" }
cosmwasm-vm-derive = { version = "3.0.2" }
cw-schema = { version = "3.0.2" }
cw-schema-derive = { version = "3.0.2" }
cosmwasm-check = { version = "3.0.2" }
go-gen = { path = "./packages/go-gen" }
schemars = "1.2.1"
serde = { version = "1.0.228", default-features = false, features = ["alloc", "derive"] }
serde_json = "1.0.149"
thiserror = "2.0.18"
"#;

const EXPECTED_STDOUT: &str = r#"
Publish version: 3.0.2

Publish crates:
cw-schema-derive  v3.0.2  packages/cw-schema-derive
cw-schema  v3.0.2  packages/cw-schema
cosmwasm-vm-derive  v3.0.2  packages/vm-derive
cosmwasm-schema-derive  v3.0.2  packages/schema-derive
cosmwasm-schema  v3.0.2  packages/schema
cosmwasm-derive  v3.0.2  packages/derive
cosmwasm-core  v3.0.2  packages/core
cosmwasm-crypto  v3.0.2  packages/crypto
cosmwasm-std  v3.0.2  packages/std
cosmwasm-vm  v3.0.2  packages/vm
cosmwasm-check  v3.0.2  packages/check


  DRY-RUN   cw-schema-derive v3.0.2 packages/cw-schema-derive

  PUBLISH   cw-schema-derive v3.0.2 packages/cw-schema-derive

  DRY-RUN   cw-schema v3.0.2 packages/cw-schema

  PUBLISH   cw-schema v3.0.2 packages/cw-schema

  DRY-RUN   cosmwasm-vm-derive v3.0.2 packages/vm-derive

  PUBLISH   cosmwasm-vm-derive v3.0.2 packages/vm-derive

  DRY-RUN   cosmwasm-schema-derive v3.0.2 packages/schema-derive

  PUBLISH   cosmwasm-schema-derive v3.0.2 packages/schema-derive

  DRY-RUN   cosmwasm-schema v3.0.2 packages/schema

  PUBLISH   cosmwasm-schema v3.0.2 packages/schema

  DRY-RUN   cosmwasm-derive v3.0.2 packages/derive

  PUBLISH   cosmwasm-derive v3.0.2 packages/derive

  DRY-RUN   cosmwasm-core v3.0.2 packages/core

  PUBLISH   cosmwasm-core v3.0.2 packages/core

  DRY-RUN   cosmwasm-crypto v3.0.2 packages/crypto

  PUBLISH   cosmwasm-crypto v3.0.2 packages/crypto

  DRY-RUN   cosmwasm-std v3.0.2 packages/std

  PUBLISH   cosmwasm-std v3.0.2 packages/std

  DRY-RUN   cosmwasm-vm v3.0.2 packages/vm

  PUBLISH   cosmwasm-vm v3.0.2 packages/vm

  DRY-RUN   cosmwasm-check v3.0.2 packages/check

  PUBLISH   cosmwasm-check v3.0.2 packages/check
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
  cli_assert::command!().code(0).arg("publish").arg("--dry-run").stdout(EXPECTED_STDOUT).stderr("").execute();
  // Make sure the Cargo.toml file is modified properly.
  assert_eq!(normalize(EXPECTED_FILE), std::fs::read_to_string(&original).unwrap());
  // Revert changes to Cargo.toml file.
  std::fs::copy(&backup, original).unwrap();
  std::fs::remove_file(backup).unwrap()
}

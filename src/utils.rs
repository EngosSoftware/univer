use crate::errors::{Result, UniverError};
use cargo_metadata::camino::Utf8Path;

/// Default name of Rust manifest.
pub const RUST_MANIFEST_NAME: &str = "Cargo.toml";

/// Reads the content of the file into string.
pub fn read_file(file_name: impl AsRef<Utf8Path>) -> Result<String> {
  let path = file_name.as_ref();
  std::fs::read_to_string(path).map_err(|e| UniverError::new(format!("failed to read file {}, reason: {}", path, e)))
}

/// Writes string content to file.
pub fn write_file(file_name: impl AsRef<Utf8Path>, contents: impl AsRef<str>) -> Result<()> {
  let path = file_name.as_ref();
  std::fs::write(path, contents.as_ref()).map_err(|e| UniverError::new(format!("failed to write file {}, reason: {}", path, e)))
}

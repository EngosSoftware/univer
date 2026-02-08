use crate::errors::{Result, univer_error};
use crate::model::Workspace;
use crate::utils;
use std::path::Path;

/// Switches workspace crates to local development mode.
pub fn develop(manifest_dir: &Path, _accept_all: bool, fixed_version: bool) -> Result<()> {
  let workspace = Workspace::load(manifest_dir)?;
  let mut manifest_content = utils::read_file(workspace.manifest_path())?;
  for member in &workspace.members {
    let dependency_with_version = &member.dependency_with_version(fixed_version);
    let dependency_with_path = &member.dependency_with_path();
    if manifest_content.contains(dependency_with_version) {
      manifest_content = manifest_content.replace(dependency_with_version, dependency_with_path);
    } else {
      return Err(univer_error!(
        "dependency '{}' with version '{}' not found or has an invalid format, expected '{}'",
        member.name,
        member.version,
        dependency_with_version
      ));
    }
  }
  utils::write_file(workspace.manifest_path(), manifest_content)?;
  Ok(())
}

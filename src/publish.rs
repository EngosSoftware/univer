use crate::errors::*;
use crate::model::Workspace;
use crate::utils;
use std::path::Path;

pub fn publish(manifest_dir: &Path, _dry_run: bool, _accept_all: bool) -> Result<()> {
  let workspace = Workspace::load(manifest_dir)?;
  let mut manifest_content = utils::read_file(workspace.manifest_path())?;

  let mut members_to_publish = vec![];

  for member in &workspace.members {
    let dependency_with_path = &member.dependency_with_path();
    let dependency_with_version = &member.dependency_with_version();
    if manifest_content.contains(dependency_with_path) {
      members_to_publish.push(member.clone());
    } else if !manifest_content.contains(dependency_with_version) {
      return Err(univer_error!(
        "dependency '{}' with path '{}' not found or has an invalid format, expected '{}'",
        member.name,
        member.version,
        dependency_with_path
      ));
    }
  }

  for member in &members_to_publish {
    let dependency_with_path = &member.dependency_with_path();
    let dependency_with_version = &member.dependency_with_version();
    manifest_content = manifest_content.replace(dependency_with_path, dependency_with_version);
    utils::write_file(workspace.manifest_path(), &manifest_content)?;
  }

  Ok(())
}

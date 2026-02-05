use crate::errors::*;
use crate::utils::RUST_MANIFEST_NAME;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::Utf8PathBuf;
use std::path::Path;

struct Member {
  /// Package name.
  name: String,
  /// Path to manifest file.
  manifest_path: Utf8PathBuf,
  /// Directory containing manifest file.
  manifest_dir: Utf8PathBuf,
  /// Package path.
  path: Utf8PathBuf,
  /// Publish flag.
  publish: bool,
  /// Dependencies to other members.
  dependencies: Vec<Dependency>,
}

struct Dependency {
  /// Package name.
  name: String,
}

pub fn develop(manifest_dir: &Path, accept_all: bool) -> Result<()> {
  let manifest_path = manifest_dir.join(RUST_MANIFEST_NAME);
  let mut metadata_command = MetadataCommand::new();
  metadata_command.manifest_path(manifest_path);
  let metadata = metadata_command.exec().map_err(|e| UniverError::new(format!("{}", e)))?;

  let mut members = vec![];
  let workspace_root = &metadata.workspace_root;
  let member_names = metadata.workspace_packages().iter().map(|p| p.name.to_string()).collect::<Vec<String>>();
  for package in metadata.workspace_packages() {
    let package_manifest_path = &package.manifest_path;
    let Some(package_manifest_dir) = package_manifest_path.parent() else {
      return Err(UniverError::new("no parent path"));
    };
    let package_path = package_manifest_dir.strip_prefix(workspace_root).map_err(|e| UniverError::new(format!("{}", e)))?;
    let package_publish = package.publish.as_ref().map(|v| !v.is_empty()).unwrap_or(true);

    let mut dependencies = vec![];
    for dependency in &package.dependencies {
      if member_names.contains(&dependency.name) {
        dependencies.push(Dependency { name: dependency.name.clone() })
      }
    }

    members.push(Member {
      name: package.name.to_string(),
      manifest_path: package_manifest_path.into(),
      manifest_dir: package_manifest_dir.into(),
      path: package_path.into(),
      publish: package_publish,
      dependencies,
    });
  }

  for member in members {
    println!("{:30} {} {} {} {}", member.name, member.manifest_path, member.manifest_dir, member.path, member.publish);
    for dependency in member.dependencies {
      println!("  - {:26}", dependency.name);
    }
  }

  println!();

  if accept_all { Ok(()) } else { Err(UniverError::new("no crates to develop")) }
}

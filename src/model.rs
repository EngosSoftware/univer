use crate::errors::{Result, UniverError};
use crate::utils::RUST_MANIFEST_NAME;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::Utf8PathBuf;
use std::path::Path;

pub struct Dependency {
  /// Package name.
  pub name: String,
}

pub struct Member {
  /// Package name.
  pub name: String,
  /// Path to manifest file.
  pub manifest_path: Utf8PathBuf,
  /// Directory containing manifest file.
  pub manifest_dir: Utf8PathBuf,
  /// Package path.
  pub path: Utf8PathBuf,
  /// Publish flag.
  pub publish: bool,
  /// Dependencies to other members.
  pub dependencies: Vec<Dependency>,
}

pub struct Workspace {
  /// Workspace root directory.
  pub root: Utf8PathBuf,
  /// Workspace members (publishable).
  pub members: Vec<Member>,
}

impl Workspace {
  /// Loads workspace metadata.
  pub fn load(manifest_dir: &Path) -> Result<Self> {
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
      if package_publish {
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
    }
    Ok(Self {
      root: workspace_root.clone(),
      members,
    })
  }
}

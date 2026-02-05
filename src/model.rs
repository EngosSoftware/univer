use crate::errors::{Result, UniverError, univer_error};
use crate::utils;
use crate::utils::RUST_MANIFEST_NAME;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use std::path::Path;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
  /// Package name.
  pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Member {
  /// Package name.
  pub name: String,
  /// Package version.
  pub version: String,
  /// Path to manifest file.
  pub manifest_path: Utf8PathBuf,
  /// Directory containing manifest file.
  pub manifest_dir: Utf8PathBuf,
  /// Package path.
  pub path: String,
  /// Dependencies to other members.
  pub dependencies: Vec<Dependency>,
}

impl Member {
  /// Returns the dependency prefix with version number.
  pub fn dependency_with_version(&self) -> String {
    format!("{} = {{ version = \"{}\"", self.name, self.version)
  }

  /// Returns the dependency prefix with local path.
  pub fn dependency_with_path(&self) -> String {
    format!("{} = {{ path = \"{}\"", self.name, self.path)
  }
}

#[derive(Debug, Default, Clone)]
pub struct Workspace {
  /// Path to workspace manifest file.
  pub manifest_path: Utf8PathBuf,
  /// Workspace members (publishable).
  pub members: Vec<Member>,
}

impl Workspace {
  pub fn manifest_path(&self) -> &Utf8Path {
    &self.manifest_path
  }

  /// Loads workspace metadata.
  pub fn load(manifest_dir: &Path) -> Result<Self> {
    let manifest_path = manifest_dir.join(RUST_MANIFEST_NAME);
    // Perform custom validations.
    validate(&manifest_path)?;
    // Load metadata.
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
          version: package.version.to_string(),
          manifest_path: package_manifest_path.into(),
          manifest_dir: package_manifest_dir.into(),
          path: package_path.to_string().replace("\\", "/"),
          dependencies,
        });
      }
    }
    Ok(Self {
      manifest_path: workspace_root.join(RUST_MANIFEST_NAME),
      members,
    })
  }
}

fn validate(manifest_path: &Path) -> Result<()> {
  let manifest_toml = utils::parse_toml(manifest_path)?;
  // Check if the manifest file is a workspace (required).
  let Some(workspace) = manifest_toml.get("workspace") else {
    return Err(univer_error!("missing [workspace] table"));
  };
  // Check if the workspace manifest has a package section (required).
  let Some(package) = workspace.get("package") else {
    return Err(univer_error!("missing [workspace.package] table"));
  };
  // Check if the workspace manifest has defined the version to be published (required).
  let Some(version) = package.get("version") else {
    return Err(univer_error!("missing 'version' in [workspace.package] table"));
  };
  // Check if the version is a string (required).
  let Some(version) = version.as_str() else {
    return Err(univer_error!("'version' is not a string in [workspace.package] table"));
  };
  // Get the version defined in [workspace.package].
  let _version = version.to_string();
  Ok(())
}

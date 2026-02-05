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
  /// Version defined in the `[workspace.package]`.
  version: String,
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
    // Perform custom validations on workspace manifest.
    let workspace_version = validate_workspace(&manifest_path)?;
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
    for member in &members {
      // Perform custom validations on each member.
      validate_member(member)?;
    }
    Ok(Self {
      version: workspace_version,
      manifest_path: workspace_root.join(RUST_MANIFEST_NAME),
      members,
    })
  }
}

fn validate_workspace(manifest_path: &Path) -> Result<String> {
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
  let version = version.to_string();
  // Check if the workspace has dependencies table (required).
  let Some(dependencies_table) = workspace.get("dependencies") else {
    return Err(univer_error!("missing [workspace.dependencies] table"));
  };
  // Check if dependencies is a table (required).
  let Some(dependencies_table) = dependencies_table.as_table() else {
    return Err(univer_error!("[workspace.dependencies] is not a table"));
  };
  // Check if no both 'path' and 'version' are set for dependency.
  for (name, value) in dependencies_table {
    let mut opt_path = None;
    if let Some(path) = value.get("path") {
      let Some(path) = path.as_str() else {
        return Err(univer_error!("'path' is not a string for '{}' in [workspace.dependencies] table", name));
      };
      opt_path = Some(path);
    }
    let mut opt_version = None;
    if let Some(version) = value.get("version") {
      let Some(version) = version.as_str() else {
        return Err(univer_error!("'version' is not a string for '{}' in [workspace.dependencies] table", name));
      };
      opt_version = Some(version);
    }
    if opt_path.is_some() && opt_version.is_some() {
      return Err(univer_error!("dependency '{}' has 'path' and 'version' set in [workspace.dependencies] table", name));
    }
  }
  Ok(version)
}

fn validate_member(member: &Member) -> Result<()> {
  let manifest_toml = utils::parse_toml(&member.manifest_path)?;
  let Some(package) = manifest_toml.get("package") else {
    return Err(univer_error!("missing [package] section in manifest for dependency '{}'", member.name));
  };
  let Some(package_version) = package.get("version") else {
    return Err(univer_error!("missing [package].version attribute in manifest for dependency '{}'", member.name));
  };
  let Some(package_version_workspace) = package_version.get("workspace") else {
    return Err(univer_error!("missing [package].version.workspace attribute in manifest for dependency '{}'", member.name));
  };
  let Some(package_version_workspace_value) = package_version_workspace.as_bool() else {
    return Err(univer_error!("invalid [package].version.workspace attribute in manifest for dependency '{}'", member.name));
  };
  if !package_version_workspace_value {
    return Err(univer_error!("[package].version.workspace attribute in crate '{}' must have value 'true'", member.name));
  }
  if let Some(dependencies) = manifest_toml.get("dependencies") {
    let Some(dependencies_table) = dependencies.as_table() else {
      return Err(univer_error!("[dependencies] section is not a table in crate '{}'", member.name));
    };
    validate_crate_dependencies(dependencies_table, member)?;
  }
  if let Some(dev_dependencies) = manifest_toml.get("dev-dependencies") {
    let Some(dev_dependencies_table) = dev_dependencies.as_table() else {
      return Err(univer_error!("[dev-dependencies] section is not a table in crate '{}'", member.name));
    };
    validate_crate_dependencies(dev_dependencies_table, member)?;
  }
  Ok(())
}

fn validate_crate_dependencies(dependencies: &toml::Table, member: &Member) -> Result<()> {
  // Iterate over all dependencies defined in the table.
  for (key, value) in dependencies {
    // Iterate over all member's dependencies.
    for dependency in &member.dependencies {
      if key == &dependency.name {
        // Make sure the dependency is defined in the workspace manifest.
        let Some(crate_dependency_workspace) = value.get("workspace") else {
          return Err(univer_error!("missing dependency {key}.workspace attribute in crate '{}'", member.name));
        };
        // Make sure the workspace dependency is a boolean type.
        let Some(crate_dependency_workspace_value) = crate_dependency_workspace.as_bool() else {
          return Err(univer_error!("invalid dependency {key}.workspace attribute in crate '{}'", member.name));
        };
        // Make sure the workspace dependency has value 'true'.
        if !crate_dependency_workspace_value {
          return Err(univer_error!("dependency {key}.workspace attribute in crate '{}' must have value 'true'", member.name));
        }
        // Make sure that the workspace dependency has no 'version' attribute set.
        if value.get("version").is_some() {
          return Err(univer_error!("'{key}' dependency must not have 'version' attribute set in crate '{}'", member.name));
        };
        // Make sure that the workspace dependency has no 'path' attribute set.
        if value.get("path").is_some() {
          return Err(univer_error!("'{key}' dependency must not have 'path' attribute set in crate '{}'", member.name));
        };
      }
    }
  }
  Ok(())
}

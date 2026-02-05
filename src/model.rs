use crate::errors::{Result, UniverError};
use crate::utils::RUST_MANIFEST_NAME;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::path::Path;

pub struct Dependency {
  /// Package name.
  pub name: String,
}

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
  pub path: Utf8PathBuf,
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
          path: package_path.into(),
          dependencies,
        });
      }
    }
    Ok(Self {
      manifest_path: workspace_root.join(RUST_MANIFEST_NAME),
      members,
    })
  }

  /// Returns the names of workspace members sorted in the publishing order.
  pub fn publish_order(&self) -> Vec<String> {
    let mut graph = DiGraph::<String, ()>::new();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    // Add nodes.
    for member in &self.members {
      let node_index = graph.add_node(member.name.clone());
      nodes.insert(member.name.clone(), node_index);
    }
    // Add edges.
    for member in &self.members {
      let member_node_index = nodes.get(&member.name).unwrap();
      for dependency in &member.dependencies {
        let dependency_node_index = nodes.get(&dependency.name).unwrap();
        graph.add_edge(*dependency_node_index, *member_node_index, ());
      }
    }
    let mut names = vec![];
    let node_indexes = petgraph::algo::toposort(&graph, None).unwrap();
    for node_index in node_indexes {
      let name = graph.node_weight(node_index).unwrap().to_string();
      names.push(name);
    }
    names
  }
}

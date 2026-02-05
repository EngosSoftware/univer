use crate::errors::*;
use crate::model::Workspace;
use std::path::Path;

pub fn develop(manifest_dir: &Path, accept_all: bool) -> Result<()> {
  let _workspace = Workspace::load(manifest_dir)?;

  // let names = workspace.publish_order();
  // println!("{:?}", names);
  //
  // for member in &workspace.members {
  //   println!("{:30} {} {} {} {}", member.name, member.manifest_path, member.manifest_dir, member.path, member.publish);
  //   for dependency in &member.dependencies {
  //     println!("  - {:26}", dependency.name);
  //   }
  // }
  //
  // println!();

  Ok(())
}

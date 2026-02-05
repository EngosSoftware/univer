use crate::errors::*;
use crate::model::Workspace;
use crate::utils;
use antex::{StyledText, auto};
use std::ffi::OsStr;
use std::path::Path;

pub fn publish(manifest_dir: &Path, dry_run: bool, accept_all: bool) -> Result<()> {
  let workspace = Workspace::load(manifest_dir)?;
  let mut manifest_content = utils::read_file(workspace.manifest_path())?;
  // Select members with path to be published.
  let mut members_to_publish = vec![];
  for member in &workspace.members {
    let dependency_with_path = &member.dependency_with_path();
    let dependency_with_version = &member.dependency_with_version();
    if manifest_content.contains(dependency_with_path) {
      members_to_publish.push(member.clone());
    } else if !manifest_content.contains(dependency_with_version) {
      return Err(univer_error!(
        "dependency '{}' with path '{}' not found or has an invalid format, expected: {}",
        member.name,
        member.path,
        dependency_with_path
      ));
    }
  }
  // Check if there are any crates to publish.
  if members_to_publish.is_empty() {
    return Err(univer_error!("no crates to publish"));
  }
  // Sort crates in the order of publishing.
  let members_to_publish = utils::sort(members_to_publish);
  // Ask if the version to be published is correct.
  println!();
  println!("Publish version: {}", auto().bold().green().s(workspace.version()).clear());
  if !dry_run && !utils::prompt("Is this version correct?", accept_all)? {
    return Ok(());
  }
  // List all the crates to be published with versions and ask if the list is correct.
  println!();
  println!("Publish crates:");
  for member in &members_to_publish {
    println!(
      "{}  {}  {}",
      auto().bold().blue().s(&member.name).clear(),
      auto().bold().green().s('v').s(workspace.version()).clear(),
      member.path
    );
  }
  println!();
  if !dry_run && !utils::prompt("Do you want to publish all these crates?", accept_all)? {
    return Ok(());
  }

  // Publish crates.
  for member in &members_to_publish {
    // Ask if perform dry-run before publishing.
    println!(
      "\n{} {} {} {}",
      auto().bold().bg_yellow().s("  DRY-RUN  ").clear(),
      auto().bold().blue().s(&member.name).clear(),
      auto().bold().green().s('v').s(workspace.version()).clear(),
      member.path
    );
    if !dry_run && utils::prompt("Perform dry-run before publishing this crate?", accept_all)? {
      execute_command("cargo", ["publish", "--dry-run", "--color=always"], &member.manifest_dir)?;
    }
    // Ask if publish the crate.
    println!(
      "\n{} {} {} {}",
      auto().bold().bg_red().s("  PUBLISH  ").clear(),
      auto().bold().blue().s(&member.name).clear(),
      auto().bold().green().s('v').s(workspace.version()).clear(),
      member.path
    );
    if !dry_run && utils::prompt("Publish this crate?", accept_all)? {
      execute_command("cargo", ["publish", "--color=always"], &member.manifest_dir)?;
    }
    let dependency_with_path = &member.dependency_with_path();
    let dependency_with_version = &member.dependency_with_version();
    manifest_content = manifest_content.replace(dependency_with_path, dependency_with_version);
    utils::write_file(workspace.manifest_path(), &manifest_content)?;
  }

  Ok(())
}

fn execute_command<S, A, P>(program: S, args: A, dir: P) -> Result<()>
where
  S: AsRef<OsStr>,
  A: IntoIterator<Item = S>,
  P: AsRef<Path>,
{
  let mut command = std::process::Command::new(program);
  let mut child = command
    .args(args)
    .current_dir(dir)
    .stdin(std::process::Stdio::inherit())
    .stdout(std::process::Stdio::inherit())
    .stderr(std::process::Stdio::inherit())
    .spawn()
    .map_err(|e| univer_error!("{}", e))?;
  let exit_status = child.wait().map_err(|e| univer_error!("{}", e))?;
  if !exit_status.success() {
    return Err(univer_error!("executing command failed with status code: {}", exit_status));
  }
  Ok(())
}

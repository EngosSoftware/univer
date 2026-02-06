use crate::errors::{Result, univer_error};
use crate::model::Member;
use antex::{StyledText, Text, auto};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::path::PathBuf;

/// Default name of Rust manifest.
pub const RUST_MANIFEST_NAME: &str = "Cargo.toml";

/// Reads the content of the file into string.
pub fn read_file(file_name: impl Into<PathBuf>) -> Result<String> {
  let path = file_name.into();
  std::fs::read_to_string(&path).map_err(|e| univer_error!("failed to read text file {}, reason: {}", path.display(), e))
}

/// Writes string content to file.
pub fn write_file(file_name: impl Into<PathBuf>, contents: impl AsRef<str>) -> Result<()> {
  let path = file_name.into();
  std::fs::write(&path, contents.as_ref()).map_err(|e| univer_error!("failed to write text file {}, reason: {}", path.display(), e))
}

/// Parses TOML file.
pub fn parse_toml(file_name: impl Into<PathBuf>) -> Result<toml::Value> {
  let path = file_name.into();
  toml::from_str(&read_file(&path)?).map_err(|e| univer_error!("failed to parse TOML file {}, reason {}", path.display(), e.to_string()))
}

/// Returns members sorted in the publishing order.
pub fn sort(members: Vec<Member>) -> Vec<Member> {
  let mut graph = DiGraph::<Member, ()>::new();
  let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
  // Add nodes.
  for member in &members {
    let node_index = graph.add_node(member.clone());
    nodes.insert(member.name.clone(), node_index);
  }
  // Add edges.
  for member in &members {
    let member_node_index = nodes.get(&member.name).unwrap();
    for dependency in &member.dependencies {
      let dependency_node_index = nodes.get(&dependency.name).unwrap();
      graph.add_edge(*dependency_node_index, *member_node_index, ());
    }
  }
  let mut sorted_members = vec![];
  let node_indexes = petgraph::algo::toposort(&graph, None).unwrap();
  for node_index in node_indexes {
    let member = graph.node_weight(node_index).unwrap().to_owned();
    sorted_members.push(member);
  }
  sorted_members
}

pub fn prompt(message: &str, accept: bool) -> Result<bool> {
  #[rustfmt::skip]
  fn prompt_text(prompt: &str) -> Text {
    auto().bold().s(prompt).normal().s(" [").bold().underline().s('Y').normal().s("es/").bold().underline().s('N').normal().s("o/").bold().underline().s('A').normal().s("bort]: ")
  }
  if accept {
    return Ok(true);
  }
  loop {
    print!("{}", prompt_text(message));
    io::stdout().flush().map_err(|e| univer_error!("failed to flush stdout, reason: {}", e))?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| univer_error!("failed to read line, reason: {}", e))?;
    match input.trim() {
      "Y" => return Ok(true),
      "N" => return Ok(false),
      "A" => std::process::exit(1),
      _ => println!("Please enter 'Y', 'N' or 'A'"),
    }
  }
}

use crate::errors::*;

pub fn develop(working_dir: &str, accept_all: bool) -> Result<()> {
  _ = (working_dir, accept_all);
  if accept_all { Ok(()) } else { Err(UniverError::new("no crates to develop")) }
}

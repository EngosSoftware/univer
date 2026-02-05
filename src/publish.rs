use crate::errors::*;

pub fn publish(working_dir: &str, dry_run: bool, accept_all: bool) -> Result<()> {
  _ = (working_dir, dry_run, accept_all);
  if dry_run { Ok(()) } else { Err(UniverError::new("no crates to publish")) }
}

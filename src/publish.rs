use crate::errors::*;

pub fn publish(working_dir: &str, timeout: u64, accept_all: bool, simulation: bool) -> Result<()> {
  _ = (working_dir, timeout, accept_all, simulation);
  if simulation {
    Ok(())
  } else {
    Err(UniverError::new("no crates to publish"))
  }
}

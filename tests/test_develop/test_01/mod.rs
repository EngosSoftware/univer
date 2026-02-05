use std::path::Path;
use univer::develop;

#[test]
fn a() {
  match develop(Path::new("/Users/ddepta/Work/CosmWasm/cosmwasm"), true) {
    Ok(()) => {
      println!("Development mode");
    }
    Err(reason) => {
      eprintln!("ERROR: {}", reason);
    }
  }
}

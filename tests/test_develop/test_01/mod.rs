#[test]
fn _0001() {
  cli_assert::command!().code(0).arg("develop").stdout("").stderr("").execute();
}

// #[test]
// fn a() {
//   use std::path::Path;
//   use univer::develop;
//   match develop(Path::new("/Users/ddepta/Work/CosmWasm/cosmwasm"), true) {
//     Ok(()) => {
//       println!("Development mode");
//     }
//     Err(reason) => {
//       eprintln!("ERROR: {}", reason);
//     }
//   }
// }

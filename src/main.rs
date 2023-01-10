use crate::cli::{cli, Cli};

mod cli;

pub fn main() {
  let args: Vec<String> = std::env::args().collect();
  let options = Cli {
    command: args[1].clone(),
    args: args[2..].to_vec(),
  };
  cli(options);
}

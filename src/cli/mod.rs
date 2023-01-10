mod start;
use napi_derive::napi;

pub fn cli(options: Cli) {
  // check command and run start if it is start
  match options.command.as_str() {
    "start" => start::run(),
    _ => println!("Command not found"),
  }
  start::run();
}

#[napi(object)]
pub struct Cli {
  pub command: String,
  pub args: Vec<String>,
}

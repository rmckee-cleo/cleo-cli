#![deny(clippy::all)]

mod cli;

#[macro_use]
extern crate napi_derive;

use crate::cli::Cli;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn main(options: Cli) {
  cli::cli(options);
}

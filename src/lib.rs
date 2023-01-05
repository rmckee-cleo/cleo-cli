#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn fibonacci(input: u32) -> u32 {
  if input <= 1 {
    return input;
  }
  fibonacci(input - 1) + fibonacci(input - 2)
}
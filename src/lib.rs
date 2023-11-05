#![deny(clippy::all)]

mod config;
pub mod install;
pub mod uninstall;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn command(command: Option<&str>, version: Option<&str>) {
  match command {
    Some("install") => {
      install::install(version.unwrap());
    }
    _ => {
      println!("unknown command:");
    }
  }
}

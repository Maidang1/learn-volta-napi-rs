use std::{env, path::PathBuf};
pub enum Version {
  Public(String),
}

pub struct Config {
  pub node: Version,
}

pub fn nemo_home() -> Option<PathBuf> {
  let res = env::home_dir();
  res.map(|home| home.join(".nemo"))
}

pub fn node_install_root() -> Option<PathBuf> {
  nemo_home().map(|nemo| nemo.join("versions").join("node"))
}

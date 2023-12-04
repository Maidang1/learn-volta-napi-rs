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

const PUBLIC_NODE_SERVER_ROOT: &'static str = "https://nodejs.org/dist/";

pub fn public_node_url(version: &str, os: &str, arch: &str) -> String {
  let verbose_root = format!("node-v{}-{}-{}", version, os, arch);

  format!(
    "{}/v{}/{}.tar.gz",
    PUBLIC_NODE_SERVER_ROOT, version, verbose_root
  )
}

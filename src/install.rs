use crate::config;

pub fn install(version: &str) -> Option<()> {
  println!("install command {}", version);
  let dest = config::node_install_root().unwrap();
  Some(())
}

use crate::config;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, RANGE};
use reqwest::Response;
use std::{env::consts::ARCH, path::Path};
use term_size;
use tokio;

#[cfg(target_os = "macos")]
const OS: &'static str = "darwin";

#[cfg(target_os = "linux")]
const OS: &'static str = "linux";

#[cfg(target_os = "windows")]
const OS: &'static str = "win";

#[cfg(target_arch = "x86")]
const ARCH: &'static str = "x86";

#[cfg(target_arch = "x86_64")]
const ARCH: &'static str = "x64";

pub async fn by_version(dest: &Path, version: &str) {
  let verbose_root = format!("node-v{}-{}-{}", version, OS, ARCH);
  let concise_root = format!("v{}", version);

  let url = config::public_node_url(version, OS, ARCH);
  let uncompressed_len = gunzipped_content_length(&url).unwrap_or(0);

  let response = reqwest::get(&url).await.unwrap();

  if !response.status().is_success() {
    panic!("failed response: {:?}", response.status());
  }
  let compressed_len = response
    .headers()
    .get(CONTENT_LENGTH)
    .and_then(|value| value.to_str().ok())
    .and_then(|value| value.parse().ok())
    .unwrap_or(0);
  let final_len = if uncompressed_len > 0 {uncompressed_len} else {compressed_len};
  let bar = progress_bar(
    &format!("Installing v{}", version),
    final_len
  );

  if uncompressed_len
}

#[tokio::main]
async fn gunzipped_content_length(url: &str) -> Result<u64, reqwest::Error> {
  let client = reqwest::Client::new();
  let response = client.head(url).send().await?;
  let content_length = response
    .headers()
    .get(CONTENT_LENGTH)
    .and_then(|value| value.to_str().ok())
    .and_then(|value| value.parse().ok())
    .unwrap_or(0);
  Ok(content_length)
}

fn progress_bar(msg: &str, len: u64) -> ProgressBar {
  let display_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
  let msg_width = msg.len();
  let available_width = display_width - 2 - msg_width - 2 - 2 - 1 - 3 - 1;
  let bar_width = ::std::cmp::min(available_width, 40);
  let bar = ProgressBar::new(len);
  bar.set_message(String::from(msg));
  bar.set_style(
    ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
      .unwrap()
      .progress_chars("##-"),
  );
  bar
}

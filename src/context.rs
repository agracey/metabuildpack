use std::path::PathBuf;

use std::collections::HashMap;

pub struct Context {
  pub build_id: String,
  pub app_name: String,

  pub environment: HashMap<String, String>,


  pub layers_dir: PathBuf,
  pub env_dir: PathBuf,
  pub plan_file: PathBuf,
  pub staging_dir: PathBuf,
  pub buildpack_dir: PathBuf
}

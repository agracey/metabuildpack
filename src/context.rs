use std::path::PathBuf;
use handlebars::Handlebars; 

use std::collections::HashMap;
use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize, Clone)]
pub struct Context {
  pub build_id: String,
  pub app_name: String,
  pub buildpack_name: String,

  pub environment: HashMap<String, String>,


  pub layers_dir: PathBuf,
  pub env_dir: PathBuf,
  pub plan_file: PathBuf,
  pub staging_dir: PathBuf,
  pub buildpack_dir: PathBuf,

}

// impl Context {
//   pub fn renderIntoString(&self, String: templ)->String {
//     let mut handlebars = Handlebars::new();
//     handlebars.render_template_with_context(templ, self).unwrap()
//   }
// }


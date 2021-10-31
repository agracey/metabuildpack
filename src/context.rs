use std::path::PathBuf;
use handlebars::Handlebars; 

use std::collections::HashMap;
use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize, Clone)]
pub struct Context {
  pub build_id: String,
  pub app_name: String,
  pub buildpack_name: String,

  pub env: HashMap<String, String>,


  pub layers_dir: PathBuf,
  pub env_dir: PathBuf,
  pub plan_file: PathBuf,
  pub staging_dir: PathBuf,
  pub buildpack_dir: PathBuf,

}

impl Context {

  fn into(self:&Self)->handlebars::Context {
    handlebars::Context::wraps(self).unwrap()

  }

  pub fn renderIntoString(self:&Self, templ: String)->String {

    let mut handlebars = Handlebars::new();
    handlebars.render_template_with_context(templ.as_str(), &self.into()).unwrap()
  }
}


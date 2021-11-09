use std::path::PathBuf;
use handlebars::Handlebars; 

use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use std::fs;

use crate::buildspec;
use crate::context;

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

  pub fn build(args: &clap::ArgMatches, spec: buildspec::Buildspec) -> Self {

    let initial_env = Self::build_initial_env(&args);

    let ctx = context::Context{
        app_name: "Some App".to_string(), // Do we know this?
        build_id: "Some App".to_string(), // Do we know this?
        buildpack_name: spec.name.clone(),
        env: initial_env,
        layers_dir: PathBuf::from(args.value_of("layers").unwrap()),
        env_dir: PathBuf::from(args.value_of("envdir").unwrap()),
        plan_file: PathBuf::from(args.value_of("plan").unwrap()),
        staging_dir: PathBuf::from(args.value_of("workingdir").unwrap()),
        buildpack_dir: PathBuf::from(args.value_of("buildpackdir").unwrap())
    };

    
    let rendered_env = Self::build_rendered_env(&args, spec.clone(), ctx);
    context::Context{
      app_name: "Some App".to_string(), // Do we know this?
      build_id: "Some App".to_string(), // Do we know this?
      buildpack_name: spec.name.clone(),
      env: rendered_env,
      layers_dir: PathBuf::from(args.value_of("layers").unwrap()),
      env_dir: PathBuf::from(args.value_of("envdir").unwrap()),
      plan_file: PathBuf::from(args.value_of("plan").unwrap()),
      staging_dir: PathBuf::from(args.value_of("workingdir").unwrap()),
      buildpack_dir: PathBuf::from(args.value_of("buildpackdir").unwrap())
    }
  }

  // Fill in from spec -> project.toml -> process.env -> env_dir
  fn build_initial_env(args: &clap::ArgMatches) -> HashMap<String,String> {
    let mut env = HashMap::new();
    // Only listen for environment variables that start with BP or BPE and strip off the prefix
    for (key, val) in std::env::vars() {
        if key.starts_with("BP_") || key.starts_with("BPE_") {
            env.insert(key.replace("BP_", "BPE_").to_string(), val);
        } else {
          env.insert(key, val);
        }
    }

    //TODO persist back to env_dir (before reading from)?

    let envpath = PathBuf::from(args.value_of("envdir").unwrap());
    let envdir = std::fs::read_dir(envpath).unwrap();

    //Walk envdir and map filename/key to content/values
    for file in envdir {
        let filepath = file.unwrap().path();
        let filepath2 = filepath.clone(); // TODO: uh... I don't get it. But this works...
        let key = filepath2.file_name().unwrap().to_str().unwrap();
        let val = fs::read_to_string(filepath).unwrap();

        env.insert(key.to_string(), val);
    }

    return env;
  }
  // Fill in from spec -> project.toml -> process.env -> env_dir
  fn build_rendered_env(args: &clap::ArgMatches, spec:buildspec::Buildspec, ctx: Context) -> HashMap<String,String> {
    let mut env = HashMap::new();

    // Only listen for environment variables that start with BP or BPE and strip off the prefix
    for (key, val) in std::env::vars() {
        if key.starts_with("BP_") || key.starts_with("BPE_") {
            env.insert(key.replace("BP_", "BPE_").to_string(), val);
        } else {
          env.insert(key, val);
        }
    }

    //TODO persist back to env_dir (before reading from)?

    let envpath = PathBuf::from(args.value_of("envdir").unwrap());
    let envdir = std::fs::read_dir(envpath).unwrap();

    //Walk envdir and map filename/key to content/values
    for file in envdir {
        let filepath = file.unwrap().path();
        let filepath2 = filepath.clone(); // TODO: uh... I don't get it. But this works...
        let key = filepath2.file_name().unwrap().to_str().unwrap();
        let val = fs::read_to_string(filepath).unwrap();

        env.insert(key.to_string(), val);
    }

    for def in spec.environment {
      env.insert(def.key, ctx.render_into_string(def.default));
    }

    return env;
  }


  pub fn into(self:&Self)->handlebars::Context {
    handlebars::Context::wraps(self).unwrap()

  }

  pub fn render_into_string(self:&Self, templ: String)->String {

    let handlebars = Handlebars::new();
    handlebars.render_template_with_context(templ.as_str(), &self.into()).unwrap()
  }
}


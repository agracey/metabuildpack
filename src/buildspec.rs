use serde::{Deserialize,Serialize};
use std::fs;

#[derive(Serialize,Deserialize,Clone)]
pub struct Exists {
    pub path: String
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Contents {
    pub file: String,
    pub regex: String
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Script {
    pub command: String,
    pub allow_fail: Option<bool>
}


#[derive(Serialize,Deserialize,Clone)]
pub struct Detect {
    pub exists: Option<Vec<Exists>>,
    pub scripts: Option<Vec<Script>>
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Remote {
    pub url: String,
    pub to: String,
    pub to_dir: String
}


#[derive(Serialize,Deserialize,Clone)]
pub struct Local {
  pub from: String,
  pub to: String,
  pub to_dir: String
}



#[derive(Serialize,Deserialize,Clone)]
pub struct BuildStep {
    pub local: Option<Vec<Local>>,
    pub remote: Option<Vec<Remote>>,
    pub scripts: Option<Vec<Script>>
}

//Should these be optional?
#[derive(Serialize,Deserialize,Clone)]
pub struct Layer {
    pub name: String,
    pub cache: bool,
    pub launch: bool,
    pub build: bool
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Env {
    pub key: String,
    pub default: String
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Buildspec {
  pub name: String,
  pub layers: Vec<Layer>,
  pub environment: Vec<Env>,
  pub detect: Detect,
  pub build: Vec<BuildStep>,
  pub process: Option<String>
}



impl Buildspec {
    pub fn read_specfile(args: &clap::ArgMatches) -> Self{
        let specfile = args.value_of("specfile").unwrap();
        let specfile_contents = fs::read_to_string(specfile).expect("Cannot Read File");
        serde_json::from_str(&specfile_contents).unwrap()
    }
}
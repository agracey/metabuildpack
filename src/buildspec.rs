use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize)]
pub struct Detect {
    pub filename: String
}

#[derive(Serialize,Deserialize)]
pub struct Build {
    pub command: String
}


#[derive(Serialize,Deserialize)]
pub struct Buildspec {
  pub name: String,
  pub detect: Vec<Detect>,
  pub build: Vec<Build>
}


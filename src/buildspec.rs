use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize)]
pub struct Exists {
    pub path: String
}

#[derive(Serialize,Deserialize)]
pub struct Contents {
    pub file: String,
    pub regex: String
}

#[derive(Serialize,Deserialize)]
pub struct Script {
    pub command: String,
    pub allow_fail: Option<bool>
}


#[derive(Serialize,Deserialize)]
pub struct Detect {
    pub exists: Option<Vec<Exists>>,
    pub scripts: Option<Vec<Script>>
}





#[derive(Serialize,Deserialize)]
pub struct Remote {
    pub url: String,
    pub path: String
}


#[derive(Serialize,Deserialize)]
pub struct Local {
  pub from: String,
  pub to: String
}



#[derive(Serialize,Deserialize)]
pub struct BuildStep {
    pub local: Option<Vec<Local>>,
    pub remote: Option<Vec<Remote>>,
    pub scripts: Option<Vec<Script>>
}


#[derive(Serialize,Deserialize)]
pub struct Buildspec {
  pub name: String,
  pub detect: Detect,
  pub build: Vec<BuildStep>
}


use crate::buildspec::Detect;
use crate::scriptrun::run_script;
use crate::context::Context;

use anyhow::anyhow;
use anyhow::{Error, Result};
 
use std::path::Path;

fn check_file(filename: String) -> bool {
  println!("Checking File {}", &filename);
  Path::new(&filename).exists()
}


pub fn detect(spec: Detect, ctx: Context) -> Result<(), Error> {



    if let Some(exists) = spec.exists {
      for exist in exists {
        if !check_file(exist.path) {
          println!("File Not Found, Stopping");
          return Err(anyhow!("File Not Found"));
        }
      };
    }

    if let  Some(scripts) = spec.scripts {
      println!("Running Scripts");
      for script in scripts {
        println!("Running: {} {}", script.command, ctx.build_id);
        let (success, _, _) = run_script("./".to_string(), script.command, &ctx);

        if success {
          println!("Ran successfully");
        } else {
          return Err(anyhow!("Script Failed"));
        }
      };
    }

    Ok(())
}

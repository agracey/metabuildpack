use crate::buildspec::Detect;
use crate::scriptrun::run_script;
 
use std::path::Path;

fn check_file(filename: String) -> bool {
  println!("Checking File {}", &filename);
  Path::new(&filename).exists()
}


pub fn detect(spec: Detect) -> bool {


    if let Some(exists) = spec.exists {
      for exist in exists {
        if !check_file(exist.path) {
          println!("File Not Found, Stopping");
          return false
        }
      };
    }

    if let  Some(scripts) = spec.scripts {
      println!("Running Scripts");
      for script in scripts {
        println!("Running: {}", script.command);
        let (success, _, _) = run_script("./".to_string(), script.command);

        if success {
          println!("Ran successfully");
        } else {
          println!("Failed");
          return false
        }
      };
    }

    return true
}

use crate::buildspec::Buildspec;
 
use std::path::Path;

fn check_file(filename: String) {
  println!("Checking File {}", &filename);
  Path::new(&filename).exists();
}


pub fn detect(spec: Buildspec) {
    println!("Detecting {}", spec.name);

    for step in spec.detect {
        check_file(step.filename)
    }
}

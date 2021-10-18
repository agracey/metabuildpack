mod build;
mod detect;
mod buildspec;

use std::fs;
use clap::{Arg, App};

fn main() {
    let matches = App::new("MetaBuildpack")
        .version("0.1.0")
        .author("agracey")
        .about("Build Applications Easily")
        .arg(Arg::with_name("stage")
             .short("s").long("stage").takes_value(true).help("detect|build"))
        .arg(Arg::with_name("specfile")
             .short("f").long("filename").takes_value(true).help("buildspec"))
        .get_matches();

    let specfile = matches.value_of("specfile").unwrap();
    let specfile_contents = fs::read_to_string(specfile).expect("Cannot Read File");


    let spec: buildspec::Buildspec = serde_json::from_str(&specfile_contents).unwrap();



    let stage = matches.value_of("stage").unwrap();

    match stage {
        "detect" => {
            detect::detect(spec);
        },
        _ => {
            build::build(spec);
        }
    };

}

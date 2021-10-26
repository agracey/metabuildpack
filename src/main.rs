mod build;
mod detect;
mod buildspec;
mod scriptrun;
mod context;

use std::fs;
use clap::{Arg, App}; // StructOpt ?
use std::collections::HashMap;
use std::path::PathBuf; 


fn main() {
    let matches = App::new("MetaBuildpack")
        .version("0.1.0")
        .author("agracey")
        .about("Build Applications Easily")
        .arg(Arg::with_name("phase")
            .long("phase").takes_value(true).help("detect|build"))
        .arg(Arg::with_name("buildpackdir")
            .short("b").long("buildpackdir").takes_value(true).help("buildpackdir").default_value("./"))
        .arg(Arg::with_name("specfile")
            .short("f").long("filename").takes_value(true).help("buildspec").default_value("./"))
        .arg(Arg::with_name("envdir")
            .short("e").long("envdir").takes_value(true).help("envdir").default_value("./"))
        .arg(Arg::with_name("plan")
            .short("p").long("plan").takes_value(true).help("plan").default_value("./"))
        .arg(Arg::with_name("stagedir")
            .short("s").long("stagedir").takes_value(true).help("stagedir").default_value("./"))
        .arg(Arg::with_name("layers")
            .short("l").long("layers").takes_value(true).help("layers").default_value("./"))
        .get_matches();



    let specfile = matches.value_of("specfile").unwrap();
    let specfile_contents = fs::read_to_string(specfile).expect("Cannot Read File");


    let spec: buildspec::Buildspec = serde_json::from_str(&specfile_contents).unwrap();


    let env = HashMap::new();


    let ctx= context::Context{
        app_name: "Some App".to_string(),
        build_id: "Some App".to_string(),
        environment: env,
        layers_dir: PathBuf::from(matches.value_of("layers").unwrap()), //TODO move defaults to Clap
        env_dir: PathBuf::from(matches.value_of("envdir").unwrap()),
        plan_file: PathBuf::from(matches.value_of("plan").unwrap()),
        staging_dir: PathBuf::from(matches.value_of("stagedir").unwrap()),// Do I need unwrap?
        buildpack_dir: PathBuf::from(matches.value_of("layers").unwrap())
    };


    let stage = matches.value_of("phase").unwrap();

    match stage {
        "detect" => {
            if detect::detect(spec.detect, ctx) {
                println!("Buildpack Detected, will run");
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }

        },
        _ => {
            build::build(spec.build, ctx);
        }
    };

}

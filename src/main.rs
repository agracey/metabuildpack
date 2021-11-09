mod build;
mod detect;
mod buildspec;
mod scriptrun;
mod context;

use std::fs;
use clap::{Arg, App}; // StructOpt ?
use std::path::PathBuf; 
use std::io::{Write};

use opentelemetry::{global, trace::{ Tracer}};

/*
 metabuildpack 
   --phase detect|build 
   --filename /cnb/buildpacks/buildpackname/x.y.z/test.json 
   --envdir /platform/env (TODO, why?)
   -p /tmp/plan.somenumber/plan.toml
   --buildpackdir /cnb/buildpacks/buildpackname/x.y.z/
   --workingdir /workspace (`pwd` in ) 
   --layer /path/to/layer/

*/
fn read_cli_args()-> clap::ArgMatches<'static> {
    App::new("MetaBuildpack")
    .version("0.1.0")
    .author("agracey")
    .about("Build Applications Easily")
    .arg(Arg::with_name("phase")
        .long("phase").takes_value(true).help("detect|build"))
    .arg(Arg::with_name("buildpackdir")
        .short("b").long("buildpackdir").takes_value(true).help("directory where buildpack is mounted").default_value("./"))
    .arg(Arg::with_name("specfile")
        .short("f").long("filename").takes_value(true).help("file with buildspec included").default_value("./test.json"))
    .arg(Arg::with_name("envdir")
        .short("e").long("envdir").takes_value(true).help("envdir").default_value("./"))
    .arg(Arg::with_name("plan")
        .short("p").long("plan").takes_value(true).help("plan").default_value("./"))
    .arg(Arg::with_name("workingdir")
        .short("w").long("workingdir").takes_value(true).help("workingdir").default_value("./"))
    .arg(Arg::with_name("layers")
        .short("l").long("layers").takes_value(true).help("layers").default_value("./"))
    .get_matches() 
}


// Write config to each layer
fn setup_layers(layers: Vec<buildspec::Layer>, ctx: context::Context){
    global::tracer("my-component").in_span("write_layer_config", |_cx| {

        for layer in layers {

            let res = std::fs::create_dir_all(PathBuf::from(format!("{}/{}", ctx.layers_dir.to_str().unwrap(), layer.name)));

            if res.is_err() {
                println!("Couldn't create layer: {}", layer.name);
                return //TODO bubble error
            }

            let mut file_contents = String::new();

            file_contents.push_str("[types]\n");

            if layer.cache {
                file_contents.push_str("cache = true\n")
            }
            if layer.launch {
                file_contents.push_str("launch = true\n")
            }
            if layer.build {
                file_contents.push_str("build = true\n")
            }

            let  path = PathBuf::from(format!("{}/{}.toml", ctx.layers_dir.to_str().unwrap(), layer.name));

            let mut handle = fs::OpenOptions::new().write(true).create(true).open(path).unwrap();

            if write!(handle, "{}", file_contents).is_ok() {
                println!("Layer written: {}", layer.name);
            }
        }

    })
}


// Write config to each layer
fn write_launch(cmd:String, ctx: context::Context){
    global::tracer("my-component").in_span("write_launch_config", |_cx| {

        let mut file_contents = String::new();
        file_contents.push_str("[[processes]]\n");
        file_contents.push_str("type = \"web\"\n");
        file_contents.push_str("command = \"");
        file_contents.push_str(ctx.render_into_string(cmd).as_str()); //TODO escaping "\""
        file_contents.push_str("\"\n");

        let path = PathBuf::from(format!("{}/launch.toml", ctx.layers_dir.to_str().unwrap()));

        let mut handle = fs::OpenOptions::new().write(true).create(true).open(path).unwrap();

        if write!(handle, "{}", file_contents).is_ok() {
            println!("Launch written")
        }
    })
}

fn main(){

    //Set up comand line args
    let args = read_cli_args();

    let spec = buildspec::Buildspec::read_specfile(&args);

    let ctx = context::Context::build(&args, spec.clone());

    let mut exit_val = 0;

    match args.value_of("phase").unwrap_or("unknown") {//TODO error if not passed in
        "detect" => {
            if detect::detect(spec.detect, ctx) {
                println!("Buildpack Detected, will run");
            } else {
                exit_val=100;
            }
        },
        _ => {
            setup_layers(spec.layers, ctx.clone());
            build::build(spec.build, ctx.clone());

            if let Some(proc) = spec.process {
                write_launch(proc, ctx.clone()); 
            }
        }
    };

    std::process::exit(exit_val);
}

mod build;
mod detect;
mod buildspec;
mod scriptrun;
mod context;

use std::fs;
use clap::{Arg, App}; // StructOpt ?
use std::collections::HashMap;
use std::path::PathBuf; 
use std::io::{Write};


use opentelemetry::{global};

use opentelemetry::sdk::{trace as sdktrace};
use opentelemetry::trace::TraceError;
use opentelemetry::trace;
use opentelemetry::{
    trace::{Span, TraceContextExt, Tracer},
    Context, Key, KeyValue,
};



pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("my_app")
        .with_collector_endpoint("http://localhost:14268/api/traces")
        .install_simple();

    if tracer.is_ok() {
        println!("Tracer is OK");
    }

    tracer
}

pub fn flush_tracer() {
    global::shutdown_tracer_provider(); // sending remaining spans
}

  


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


fn read_specfile(args: &clap::ArgMatches) -> buildspec::Buildspec{
    let specfile = args.value_of("specfile").unwrap();
    let specfile_contents = fs::read_to_string(specfile).expect("Cannot Read File");
    serde_json::from_str(&specfile_contents).unwrap()
}



fn build_env(args: &clap::ArgMatches, spec:buildspec::Buildspec) -> HashMap<String,String> {
    let mut env = HashMap::new();

    for def in  spec.environment {
      env.insert(def.key,def.default);
    }

    std::env::
    //TODO fill in from project.toml, process.env, and env_dir

    return env;
}

fn build_context(args: &clap::ArgMatches, spec:buildspec::Buildspec) -> context::Context {

    let env = build_env(&args, spec.clone());
    context::Context{
        app_name: "Some App".to_string(),
        build_id: "Some App".to_string(),
        buildpack_name: spec.name,


        environment: env,
        layers_dir: PathBuf::from(args.value_of("layers").unwrap()),
        env_dir: PathBuf::from(args.value_of("envdir").unwrap()),
        plan_file: PathBuf::from(args.value_of("plan").unwrap()),
        staging_dir: PathBuf::from(args.value_of("workingdir").unwrap()),
        buildpack_dir: PathBuf::from(args.value_of("buildpackdir").unwrap())
    }
}

fn write_config(cfg: buildspec::Config, ctx: context::Context){

    let mut file_contents = String::new();

    if cfg.cache {
        file_contents.push_str("cache = true\n")
    }
    if cfg.runtime {
        file_contents.push_str("launch = true\n")
    }

    let  path = PathBuf::from(format!("{}/{}.toml", ctx.layers_dir.to_str().unwrap(), ctx.buildpack_name));

    let mut handle = fs::OpenOptions::new().write(true).create(true).open(path).unwrap();

    if write!(handle, "{}", file_contents).is_ok() {
        println!("Congig written")
    }
}

//const FOO_KEY: Key = Key::from_static_str("ex.com/foo");



fn main(){

// Tracing logic that I have no clue if I'm doing right. Multiple attempts in here...

    let ot_tracer = init_tracer().unwrap();
    let mut span1 = ot_tracer.start("my_span2");

    span1.set_attribute(KeyValue::new("attempt", "1"));

    span1.end();

    let tracer = global::tracer("my_tracer");
    let mut span2 = tracer.start("my_span2");

    span2.set_attribute(KeyValue::new("attempt", "2"));

    span2.end();


    // ot_tracer.in_span("test", |cx| {
    //     let span = cx.span();
    //     span.add_event(
    //         "Nice operation!".to_string(),
    //         vec![Key::new("bogons").i64(100)],
    //     );
        
    //     span.set_attribute(FOO_KEY.string("yes"));
    // });

    flush_tracer();

// end tracing logic that I have no clue if I'm doing right






    //Set up comand line args
    let args = read_cli_args();
    let spec: buildspec::Buildspec = read_specfile(&args);
    let ctx = build_context(&args, spec.clone());

    match args.value_of("phase").unwrap_or("unknown") {//TODO error if not passed in
        "detect" => {
            if detect::detect(spec.detect, ctx) {
                println!("Buildpack Detected, will run");
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        },
        _ => {
            write_config(spec.config, ctx.clone());
            build::build(spec.build, ctx.clone());
        }
    };
}

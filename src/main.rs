mod build;
mod detect;
mod buildspec;
mod scriptrun;
mod context;

use std::fs;
use clap::{Arg, App};
use opentelemetry::trace::Span;
use std::path::PathBuf; 
use std::io::{Write};

use anyhow::{Error, Result};

use opentelemetry::{trace::{ Tracer, TraceError}};

use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{
    trace::{TraceContextExt}, Key,
};
use opentelemetry::sdk::Resource;
use opentelemetry::{global, sdk::trace as sdktrace};
use opentelemetry_otlp::{WithExportConfig};

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
fn setup_layers(layers: Vec<buildspec::Layer>, ctx: context::Context) -> Result<(),Error>{
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


    });
    
    Ok(())
}


// Write config to each layer
fn write_launch(cmd:String, ctx: context::Context) -> Result<(),Error>{
    global::tracer("my-component").in_span("write_launch_config", |_cx| {

        let mut file_contents = String::new();
        file_contents.push_str("[[processes]]\n");
        file_contents.push_str("type = \"web\"\n");
        file_contents.push_str("command = \"");
        file_contents.push_str(ctx.render_into_string(cmd).unwrap().as_str()); //TODO escaping "\""
        file_contents.push_str("\"\n");

        let path = PathBuf::from(format!("{}/launch.toml", ctx.layers_dir.to_str().unwrap()));

        let mut handle = fs::OpenOptions::new().write(true).create(true).open(path).unwrap();

        if write!(handle, "{}", file_contents).is_ok() {
            println!("Launch written")
        }
    });
    Ok(())
}


fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {

    let config = sdktrace::config().with_resource(Resource::new(vec![
        Key::new("service.name").string("metabuildpack")
    ]));


    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://collector.linkerd-jaeger:4317"),
        ).with_trace_config(config)
        .install_batch(opentelemetry::runtime::Tokio)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{
    let _ = init_tracer()?;
    let tracer = global::tracer("metabuildpack/build");


    tracer.in_span("buildstep", |cx| {
        let span = cx.span();
        //Set up comand line args
        let args = read_cli_args();

        span.add_event("Build Phase".to_string(), 
        vec![Key::new("phase").string(args.value_of("phase").unwrap_or("unknown").to_owned())]);

        let spec = buildspec::Buildspec::read_specfile(&args);

        span.add_event("Buildpack Name".to_string(), vec![Key::new("buildpack-name").string(spec.name.to_owned())]);

        let ctx = context::Context::build(&args, spec.clone()).unwrap();

        let mut exit_val = 0;

        match args.value_of("phase").unwrap_or("unknown") {//TODO error if not passed in
            "detect" => {
                
                tracer.in_span("detect", |cx| {
                    let span = cx.span();
                    if detect::detect(spec.detect, ctx).is_ok() {
                        span.add_event("Buildpack Detected, will run".to_string(), vec![]);
                        println!("Buildpack Detected, will run");
                    } else {
                        exit_val=100;
                    }
                });
            },
            _ => {
                tracer.in_span("build", |cx| {
                    let span = cx.span();
                    if let Err(_e) = setup_layers(spec.layers, ctx.clone()) {
                        span.add_event("Error Setting up Layers".to_string(), vec![]);
                        exit_val=100;
                    } else if let Err(_e) = build::build(spec.build, ctx.clone()) {
                        span.add_event("Error Building Layer(s)".to_string(), vec![]);
                        exit_val=100;
                    } else if let Some(proc) = spec.process {
                        if let Err(_e) = write_launch(proc, ctx.clone()) {
                            span.add_event("Error Writing Launch".to_string(), vec![]);
                            exit_val=100;
                        }
                    }
                });
            }
        };

        shutdown_tracer_provider();
        std::process::exit(exit_val);
    })
}

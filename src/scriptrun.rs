use std::process::Command;
use std::str::from_utf8;
use crate::context::Context;
use opentelemetry::{global, Key};
use opentelemetry::trace::{TraceContextExt};
use opentelemetry::trace::Tracer;

pub(crate) fn run_script(working_dir: String, command: String, ctx: &Context) -> (bool, String, String) {
  let tracer = global::tracer("");
  tracer.in_span("run script", |cx| {
    let span = cx.span();

    let mut cmd = Command::new("/bin/sh");

    let rendered_command = ctx.render_into_string(command).unwrap();
    
    span.add_event("script run".to_string(), 
    vec![
      Key::new("command").string(rendered_command.clone())
    ]);

    let cmd_ret = cmd.arg("-c").arg(rendered_command).current_dir(working_dir).envs(ctx.env.clone());
    let status = cmd_ret.status().expect("failed to execute sh -c");
    let output = cmd_ret.output().expect("failed to execute sh -c");
        
    span.add_event("script out".to_string(), 
    vec![
      Key::new("success?").bool(status.success())
    ]);

    return (status.success(), from_utf8(&output.stdout).unwrap().to_string(),  from_utf8(&output.stderr).unwrap().to_string())
  })
}
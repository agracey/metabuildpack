use std::process::Command;
use std::str::from_utf8;
use crate::context::Context;

pub(crate) fn run_script(working_dir: String, command: String, ctx: &Context) -> (bool, String, String) {

  let mut cmd = Command::new("sh");
  let cmd2 = cmd.arg("-c").arg(command).current_dir(working_dir).envs(ctx.environment.clone());
  let status = cmd2.status().expect("failed to execute sh -c");
  let output = cmd2.output().expect("failed to execute sh -c");

  return (status.success(), from_utf8(&output.stdout).unwrap().to_string(),  from_utf8(&output.stderr).unwrap().to_string())
}
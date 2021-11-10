use crate::buildspec::BuildStep;
use crate::context::Context;
use crate::scriptrun::run_script;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::{Error, Result};

use curl::easy::Easy;

use fs_extra::file;
use fs_extra::dir;
use std::fs;
use std::io::Write;

fn curl_to(url: String, path: PathBuf) -> Result<(), Error> {
  println!("cURLing {:?} to {:?}", url, path.file_name());

  let mut filehandle = fs::OpenOptions::new()
  .write(true)
  .append(true)
  .create(true)
  .open(path)
  .unwrap();


  let mut handle = Easy::new();

  if let Err(e) = handle.url(&url) {
    return Err(anyhow!("Poorly Formatted URL in remote files: {:?}", e.description()))
  }

  handle.write_function(move |data| {
    if filehandle.write_all(data).is_ok(){
      return Ok(data.len())
    } else {
      return Ok(0)
    }
  })?;

  if let Err(e) = handle.perform() {
    return Err(anyhow!("Failed Curl: {:?}", e.description()))
  }
  Ok(())
}

fn move_to(from: PathBuf, to: PathBuf) -> Result<(), Error> {
  if from.is_dir(){
    let mut dir_copy_options: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions::new();
    dir_copy_options.copy_inside = true;

    println!("Copying Dir {:?} to {:?}", from.file_name(), to.file_name());
    let copy_res = dir::copy(from,to, &dir_copy_options);
    if copy_res.is_err() {
      return Err(anyhow!("Dir Copy Failed -- {}", copy_res.err().unwrap()));
    }


  } else {
    let file_copy_options: fs_extra::file::CopyOptions = fs_extra::file::CopyOptions::new();

    println!("Copying File {:?} to {:?}", from.file_name(), to.file_name());
    let copy_res =  file::copy(from, to, &file_copy_options);
    if copy_res.is_err() {
      return Err(anyhow!("File Copy Failed -- {}", copy_res.err().unwrap()));
    }
  }

  Ok(())
}

fn runscript(command: String, ctx: &Context) -> Result<(), Error> {

  println!("Running {:?}", command.clone());
  let (success, _, _) = run_script("./".to_string(), command.clone(), ctx);

  if ! success {
    return Err(anyhow!("Command Failed: {}", command.clone()))
  }
  Ok(())
}



pub fn build(steps: Vec<BuildStep>, ctx: Context) -> Result<(), Error> {

  for step in steps {
    if let Some(remote) = step.remote {
      for remotefile in remote {
        if curl_to(remotefile.url.clone(), PathBuf::from(ctx.render_into_string(remotefile.to.clone())?)).is_err() {
          return Err(anyhow!("Couldn't curl {:?} to {:?}", remotefile.url, remotefile.to ));
        }
      }
    }

    if let Some(local) = step.local {
      for localfile in local {
        let mut from = ctx.buildpack_dir.clone();
        from.push(localfile.from);
        let to = ctx.render_into_string(localfile.to)?;

        if move_to(from.clone(), PathBuf::from(to.clone())).is_err() {
          return Err(anyhow!("Couldn't move {:?} to {:?}", from, to ));
        }
      }
    }

    if let Some(scripts) = step.scripts {
      for script in scripts {
        if runscript(script.command.clone(), &ctx).is_err() {
          return Err(anyhow!("Command Failed: {}", script.command.clone()))
        }
      }
    }
  }
  return Ok(());
}

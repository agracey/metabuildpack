use crate::buildspec::BuildStep;
// use crate::buildspec::Remote;
// use crate::buildspec::Local;
use crate::context::Context;
use crate::scriptrun::run_script;
use std::path::PathBuf;

use curl::easy::Easy;

use fs_extra::file;
use fs_extra::dir;
use std::fs;
use std::io::Write;

fn curl_to(url: String, path: PathBuf) {
  println!("cURLing {:?} to {:?}", url, path.file_name());

  let mut filehandle = fs::OpenOptions::new()
  .write(true)
  .append(true)
  .create(true)
  .open(path)
  .unwrap();


  let mut handle = Easy::new();
  handle.url(&url).unwrap();

  handle.write_function(move |data| {
    if filehandle.write_all(data).is_ok(){
      return Ok(data.len())
    } else {
      return Ok(0)
    }
  }).unwrap();

  handle.perform().unwrap();
}

fn move_to(from: PathBuf, to: PathBuf) {
  if from.is_dir(){
    let mut dir_copy_options: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions::new();
    dir_copy_options.copy_inside = true;

    println!("Copying Dir {:?} to {:?}", from.file_name(), to.file_name());
    let copy_res = dir::copy(from,to, &dir_copy_options);
    if copy_res.is_err() {
      println!("Dir Copy Failed");
    }


  } else {
    let file_copy_options: fs_extra::file::CopyOptions = fs_extra::file::CopyOptions::new();

    println!("Copying File {:?} to {:?}", from.file_name(), to.file_name());
    let copy_res =  file::copy(from, to, &file_copy_options);
    if copy_res.is_err() {
      println!("File Copy Failed, {}", copy_res.err().unwrap());
    }
  }
}

fn runscript(command: String, ctx: &Context)->bool {

  println!("Running {:?}", command);
  let (success, _, _) = run_script("./".to_string(), command, ctx);

  if success {
    println!("Ran successfully");
  } else {
    println!("Failed");
    return false
  }
  return true
}



// Should move to Result<>...
pub fn build(steps: Vec<BuildStep>, ctx: Context) ->bool {


  for step in steps {
    if let Some(remote) = step.remote {
      for remotefile in remote {
        
        curl_to(remotefile.url, PathBuf::from(ctx.render_into_string(remotefile.to)));
      }
    }

    if let Some(local) = step.local {
      for localfile in local {
        let mut from = ctx.buildpack_dir.clone();
        from.push(localfile.from);

        let to = ctx.render_into_string(localfile.to);

        move_to(from, PathBuf::from(to));
      }
    }

    if let Some(scripts) = step.scripts {
      for script in scripts {
        if ! runscript(script.command, &ctx) {
          return false
        }
      }
    }
  }
  return true
}

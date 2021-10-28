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
      println!("File Copy Failed");
    }
  }
}

fn runscript(command: String)->bool {
  println!("Running {:?}", command);
  let (success, _, _) = run_script("./".to_string(), command);

  if success {
    println!("Ran successfully");
  } else {
    println!("Failed");
    return false
  }
  return true
}




fn compile_remote_to_path(to:String, to_dir:String, ctx: &Context)->PathBuf {
  match to_dir.as_str() {
    "layer"=>{
      let mut ret = ctx.layers_dir.clone();
      ret.push(to);
      return ret;
    },
    "staging"=>{
      let mut ret = ctx.layers_dir.clone();
      ret.push(to);
      return ret;
    },
    "buildpack"=>{
      let mut ret = ctx.layers_dir.clone();
      ret.push(to);
      return ret;
    },
    _=>{
      return PathBuf::from(to);
    }
  }
}


// Should move to Result<>...
pub fn build(steps: Vec<BuildStep>, ctx: Context) ->bool {


  for step in steps {
    if let Some(remote) = step.remote {
      for remotefile in remote {
        
        curl_to(remotefile.url, compile_remote_to_path(remotefile.to, remotefile.to_dir, &ctx));
      }
    }

    if let Some(local) = step.local {
      for localfile in local {
        let mut from = ctx.buildpack_dir.clone();
        from.push(localfile.from);

        move_to(from, compile_remote_to_path(localfile.to, localfile.to_dir, &ctx));
      }
    }

    if let Some(scripts) = step.scripts {
      for script in scripts {
        if ! runscript(script.command) {
          return false
        }
      }
    }
  }
  return true
}

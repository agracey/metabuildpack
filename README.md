# Build your own Cloud Native Build Pack

With the MetaBuildpack, you can easily describe what assets and processes are needed to build software across a large number of types of applications. This gives easier governance and maintainability because the surface for mistakes is smaller!

## Features

- [x] Detection based on file existence
- [x] Detection based on script return code
- [ ] Detection based on environment variables
- [x] Pull from remote
- [x] Pull from local cache (for offline buildpacks)
- [x] Run script
- [ ] Figure out what's in the plan file and how do I use it
- [x] Use context in script
- [ ] Persisting env for future buildpacks
- [x] Caching
- [ ] Bill of materials (sbom)
- [x] Launch configuration
- [-] Tracing of Pipeline (in progress)
- [x] Published builder image and stack with tool included (docker.io/atgracey/metabuildpackbase:latest)
- [ ] Automated stack builds
- - [ ] x86_64
- - [ ] aarch64
- - [ ] s390x


## How to use

I'm building a stack that anyone can use: 

```
[stack]
id = "opensuse.stack.leap15.3"
run-image = "registry.opensuse.org/home/atgracey/opensuse_leap_15.3/cnbp_run:latest"
build-image = "docker.io/atgracey/metabuildpackbuild:latest" 
``` 
Note: I'll be moving the build image to registry.opensuse.org as soon as I get the rust code built there as well.


You can use this inside your detect and build bash scripts with: 

```
#!/usr/bin/env bash

set -euo pipefail

env_dir="$1/env"
plan_path="$2"

/usr/local/bin/metabuildpack --phase detect -b $CNB_BUILDPACK_DIR -f $CNB_BUILDPACK_DIR/spec.json -e $env_dir -p $plan_path
exit $?
```

and 

```
#!/usr/bin/env bash

set -euo pipefail
layers_dir="$1"
env_dir="$2/env"
plan_path="$3"

/usr/local/bin/metabuildpack --phase build -b $CNB_BUILDPACK_DIR -f $CNB_BUILDPACK_DIR/spec.json -e $env_dir -p $plan_path -l $layers_dir

exit $?
```


With those scripts in place, the last part is to write a spec file.

I wrote this one to install node in an airgapped manner. 

Note: I'm adding comments that will break copy and paste!

```
{
  "name":"nodejs-bins", # name used to log and trace correctly 
  "layers":[{ 
    "name":"nodebin", # layers can be used to cache files across builds, expose files to the published image, or expose files to later buildpacks 
    "cache": true,
    "launch": true,
    "build": true
  }],
  "environment":[{ # This allows us to set variables in a more dynamic way, if a value is passed in to the build process by the end user, it will superscede what's defined here. 
    "key": "PATH",
    "default": "{{env.PATH}}:{{layers_dir}}/nodebin/node-v17.0.1-linux-x64/bin/"
  }], 
  "detect": { # You can decide whether to run or not based on existence of files or the result of a script.
    "exists": [{
      "path": "./package.json"
    }],
    "scripts": [] 
  },
  "build":[{ # When building, you can pull files from other places, from a set of files provided in the buildpack itself, and run scripts. 
    "remote": [],
    "local": [],
    "scripts": [{
      "command": "tar -xf {{buildpack_dir}}/assets/node.tar.xz -C {{layers_dir}}/nodebin/"
    }]
  }],
  "process":"{{layers_dir}}/nodebin/node-v17.0.1-linux-x64/bin/node index.js" # You can also set the default command the published container will run
}
```



A reference set of buildpacks can be found at: https://github.com/agracey/buildpack_example

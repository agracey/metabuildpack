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
- [ ] Tracing of Pipeline (in progress)
- [x] Published builder image and stack with tool included (docker.io/atgracey/metabuildpackbase:latest)
- [ ] Automated stack builds
- - [ ] x86_64
- - [ ] aarch64
- - [ ] s390x


## How to use

Full story to be written when I can. See ./testing/runtest.sh for clues. 


To use the program, run 
`cargo run -- --phase build -f ./noop-spec.json  -e /tmp/env/ -p /tmp/plan.json -l /tmp/`

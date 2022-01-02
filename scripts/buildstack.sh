#!/bin/bash

registry_ns="docker.io/atgracey"
label=":latest"

cargo build --release


docker build ./ -f Dockerfile.build -t $registry_ns/metabuildpackbuild$label --build-arg base_image=registry.opensuse.org/home/atgracey/opensuse_leap_15.3/cnbp_base:latest --build-arg stack_id=opensuse.stack.leap15.3

docker push $registry_ns/metabuildpackbuild$label

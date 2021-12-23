#!/bin/bash

registry_ns="docker.io/atgracey"
label=":latest"

cargo build --release


docker build ./ -f Dockerfile.base -t $registry_ns/metabuildpackbase$label
docker build ./ -f Dockerfile.build -t $registry_ns/metabuildpackbuild$label --build-arg base_image=docker.io/atgracey/metabuildpackbase$label --build-arg stack_id=atgracey.stack.test
docker build ./ -f Dockerfile.run -t $registry_ns/metabuildpackrun$label --build-arg base_image=docker.io/atgracey/metabuildpackbase$label --build-arg stack_id=atgracey.stack.test

docker push $registry_ns/metabuildpackbase$label
docker push $registry_ns/metabuildpackbuild$label
docker push $registry_ns/metabuildpackrun$label

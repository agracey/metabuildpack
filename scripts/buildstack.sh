#!/bin/bash

registry_ns="docker.io/atgracey"


cargo build --release


docker build ./ -f Dockerfile.base -t $registry_ns/builderbase
docker build ./ -f Dockerfile.build -t $registry_ns/testbpbuild --build-arg base_image=docker.io/atgracey/builderbase --build-arg stack_id=atgracey.stack.test
docker build ./ -f Dockerfile.run -t $registry_ns/testbprun --build-arg base_image=docker.io/atgracey/builderbase --build-arg stack_id=atgracey.stack.test

docker push $registry_ns/builderbase
docker push $registry_ns/testbpbuild
docker push $registry_ns/testbprun

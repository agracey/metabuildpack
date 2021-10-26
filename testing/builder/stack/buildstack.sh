docker build ./ -f Dockerfile.base -t docker.io/atgracey/builderbase
docker build ./ -f Dockerfile.build -t docker.io/atgracey/testbpbuild --build-arg base_image=docker.io/atgracey/builderbase --build-arg stack_id=atgracey.stack.test
docker build ./ -f Dockerfile.run -t docker.io/atgracey/testbprun --build-arg base_image=docker.io/atgracey/builderbase --build-arg stack_id=atgracey.stack.test

docker push docker.io/atgracey/builderbase
docker push docker.io/atgracey/testbpbuild
docker push docker.io/atgracey/testbprun

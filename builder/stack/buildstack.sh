docker build ./ -f Dockerfile.base -t atgracey/builderbase
docker build ./ -f Dockerfile.build -t atgracey/builderbuild --build-arg base_image=atgracey/builderbase
docker build ./ -f Dockerfile.run -t atgracey/builderrun --build-arg base_image=atgracey/builderbase

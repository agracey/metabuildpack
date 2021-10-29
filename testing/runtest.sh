#!/bin/bash

pushd builder/stack; bash buildstack.sh; popd
pushd builder; bash buildbuilder.sh; popd

pushd ..; cargo build --release; popd

cp ../target/release/metabuildpack ./samplebuildpack/bin

pack build testblah --path ./testapp/ --buildpack ./samplebuildpack/ --builder docker.io/atgracey/builder --env BP_SOMEKEY=somevar

rm ./samplebuildpack/bin/metabuildpack
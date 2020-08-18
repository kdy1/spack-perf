#!/usr/bin/env bash

set -eux

cargo build --release

sync

exa -al target/release/spack-cli

time -p target/release/spack-cli  ~/projects/three.js/src/Three.js -d out

# sudo cargo flamegraph --bin spack-cli
sudo flamegraph target/release/spack-cli 


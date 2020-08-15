#!/usr/bin/env bash

set -eux

cargo build --release

sync

exa -al target/release/spack-cli

time -p target/release/spack-cli  ~/projects/three.js/src/Three.js

sudo cargo flamegraph --bin spack-cli


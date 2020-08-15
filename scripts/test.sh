#!/usr/bin/env bash

set -eux

cargo build --release

sync

exa -al ../../target/release/spack-cli

time ../../target/release/spack-cli  ../../spack/integration-tests/react/src/index.tsx -d out

sudo cargo flamegraph --bin spack-cli


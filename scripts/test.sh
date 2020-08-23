#!/usr/bin/env bash

set -eux

cargo build --release

sync

ls -al target/release/spack-cli

time -p target/release/spack-cli  rxjs/src/internal/observable/dom/AjaxObservable.ts -d out

# sudo cargo flamegraph --bin spack-cli
sudo flamegraph target/release/spack-cli 


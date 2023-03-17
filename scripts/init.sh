#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment ***"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup install $(cat ./rust-toolchain)
   rustup update stable
fi

rustup target add wasm32-unknown-unknown --toolchain $(cat ./rust-toolchain)
# cargo build --release

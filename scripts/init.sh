#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment ***"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup install nightly
   rustup update stable
fi

rustup target add wasm32-unknown-unknown --toolchain nightly
# cargo build --release

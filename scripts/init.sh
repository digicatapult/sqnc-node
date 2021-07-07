#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment ***"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup install nightly-2021-03-25
   rustup update stable
fi

rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-25
# cargo build --release

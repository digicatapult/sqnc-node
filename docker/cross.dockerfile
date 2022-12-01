FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest

RUN apt-get install --assume-yes protobuf-compiler

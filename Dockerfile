# syntax=docker/dockerfile:1.2

# build base that sets up common dependencies of the build
FROM rust:alpine as build-base

RUN apk add --no-cache \
  clang clang-dev clang-libs pkgconfig bearssl-dev git \
  gcc make g++ linux-headers protobuf protobuf-dev musl-dev

RUN set -ex; \
  wget https://github.com/gruntwork-io/fetch/releases/download/v0.4.2/fetch_linux_amd64 -P /; \
  mv /fetch_linux_amd64 /fetch; \
  chmod +x /fetch; \
  /fetch --repo="https://github.com/mozilla/sccache" --tag="~>0.2.15" --release-asset="^sccache-v[0-9.]*-x86_64-unknown-linux-musl.tar.gz$" /; \
  tar -xvf /sccache-v*-x86_64-unknown-linux-musl.tar.gz -C /; \
  mv /sccache-v*-x86_64-unknown-linux-musl/sccache /sccache; \
  rm -rf /sccache-v*-x86_64-unknown-linux-musl /sccache-v*-x86_64-unknown-linux-musl.tar.gz /fetch; \
  chmod +x /sccache;

WORKDIR /build
ARG RUST_TOOLCHAIN=nightly-2021-11-09
RUN rustup install $RUST_TOOLCHAIN && \
  rustup target add wasm32-unknown-unknown --toolchain $RUST_TOOLCHAIN

FROM build-base as build

COPY . .

# RUN --mount=type=cache,mode=0755,id=sccache,target=/cache \
#     --mount=type=tmpfs,target=/build/target \
#     PROTOC=$(which protoc) \
#     PROTOC_INCLUDE=/usr/include \
#     RUSTFLAGS=-Ctarget-feature=-crt-static \
#     RUSTC_WRAPPER=/sccache \
#     SCCACHE_DIR=/cache \
#     SCCACHE_IDLE_TIMEOUT=0 \
#     cargo build --release && \
#     cp /build/target/release/vitalam-node /vitalam-node

# # build the runtime image that will actually contain the final built executable

# FROM alpine:3.12 AS runtime

# RUN apk update
# RUN apk add libgcc libstdc++

# RUN mkdir /vitalam-node /data
# COPY --from=build /vitalam-node /vitalam-node

# WORKDIR /vitalam-node

# CMD /vitalam-node/vitalam-node

# EXPOSE 30333 9933 9944

# docker run -it --rm -h node-0 -e IDENTITY=dev -e WS=true -p 30333:30333 -p 9944:9944 -p 9933:9933 vitalam-substrate-node ./run.sh

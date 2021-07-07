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

WORKDIR vitalam-node
COPY ./rust-toolchain .
RUN rustup install $(cat ./rust-toolchain) && rustup target add wasm32-unknown-unknown --toolchain $(cat ./rust-toolchain)
RUN cargo +$(cat ./rust-toolchain) install cargo-chef

# planner image to get the dependencies list that can be built seperately

FROM build-base as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build the dependencies list based on the list from "planner"

FROM build-base as dependencies

COPY --from=planner vitalam-node/recipe.json recipe.json
RUN PROTOC=$(which protoc) \
  PROTOC_INCLUDE=/usr/include \
  RUSTFLAGS=-Ctarget-feature=-crt-static \
  RUSTC_WRAPPER=/sccache \
  SCCACHE_DIR=/cache \
  cargo chef cook --release --recipe-path recipe.json && \
  rm -rf /vitalam-node

# build the vitalam-node image

FROM build-base as build

COPY . .
COPY --from=dependencies /cache /cache
COPY --from=dependencies /usr/local/cargo /usr/local/cargo

RUN PROTOC=$(which protoc) \
  PROTOC_INCLUDE=/usr/include \
  RUSTFLAGS=-Ctarget-feature=-crt-static \
  RUSTC_WRAPPER=/sccache \
  SCCACHE_DIR=/cache \
  cargo build --release

# build the runtime image that will actually contain the final built executable

FROM alpine:3.12 AS runtime

RUN apk update
RUN apk add libgcc libstdc++

RUN mkdir /vitalam-node /data
COPY --from=build /vitalam-node/target/release/vitalam-node /vitalam-node

COPY ./scripts/start-node-docker.sh /vitalam-node/run.sh

WORKDIR /vitalam-node

CMD /vitalam-node/run.sh

EXPOSE 30333 9933 9944

# docker run -it --rm -h node-0 -e IDENTITY=dev -e WS=true -p 30333:30333 -p 9944:9944 -p 9933:9933 vitalam-substrate-node ./run.sh

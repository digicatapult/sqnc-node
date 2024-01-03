# syntax=docker/dockerfile:1.6

FROM ubuntu:jammy AS setup

RUN <<EOF
apt-get update
apt-get install -y curl ca-certificates
rm -rf /var/lib/apt/lists/*
EOF

WORKDIR /tmp/

ARG TARGETPLATFORM

RUN <<EOF
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
  curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.2/fetch_linux_amd64 --output ./fetch;
  chmod +x ./fetch;
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
  curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.2/fetch_linux_arm64 --output ./fetch;
  chmod +x ./fetch;
fi
EOF

ARG DSCP_VERSION=latest
ARG DSCP_REPO=https://github.com/digicatapult/dscp-node

RUN <<EOF
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
  ./fetch --repo="${DSCP_REPO}" --tag="${DSCP_VERSION}" --release-asset="dscp-node-.*-x86_64-unknown-linux-gnu.tar.gz" ./;
  mkdir ./dscp-node;
  tar -xzf ./dscp-node-*-x86_64-unknown-linux-gnu.tar.gz -C ./dscp-node;
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
  ./fetch --repo="${DSCP_REPO}" --tag="${DSCP_VERSION}" --release-asset="dscp-node-.*-aarch64-unknown-linux-gnu.tar.gz" ./;
  mkdir ./dscp-node;
  tar -xzf ./dscp-node-*-aarch64-unknown-linux-gnu.tar.gz -C ./dscp-node;
fi
EOF

FROM ubuntu:jammy AS runtime

RUN <<EOF
apt-get update
apt-get install -y libgcc-11-dev
rm -rf /var/lib/apt/lists/*
EOF

RUN mkdir /dscp-node /data

COPY --from=setup /tmp/dscp-node /dscp-node/

WORKDIR /dscp-node

ENTRYPOINT [ "/dscp-node/dscp-node" ]

EXPOSE 30333 9933 9944

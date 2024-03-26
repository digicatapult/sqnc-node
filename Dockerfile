# syntax=docker/dockerfile:1.7

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
  curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.6/fetch_linux_amd64 --output ./fetch;
  chmod +x ./fetch;
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
  curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.6/fetch_linux_arm64 --output ./fetch;
  chmod +x ./fetch;
fi
EOF

ARG SQNC_VERSION=latest
ARG SQNC_REPO=https://github.com/digicatapult/sqnc-node

RUN <<EOF
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
  ./fetch --repo="${SQNC_REPO}" --tag="${SQNC_VERSION}" --release-asset="sqnc-node-.*-x86_64-unknown-linux-gnu.tar.gz" ./;
  mkdir ./sqnc-node;
  tar -xzf ./sqnc-node-*-x86_64-unknown-linux-gnu.tar.gz -C ./sqnc-node;
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
  ./fetch --repo="${SQNC_REPO}" --tag="${SQNC_VERSION}" --release-asset="sqnc-node-.*-aarch64-unknown-linux-gnu.tar.gz" ./;
  mkdir ./sqnc-node;
  tar -xzf ./sqnc-node-*-aarch64-unknown-linux-gnu.tar.gz -C ./sqnc-node;
fi
EOF

FROM ubuntu:jammy AS runtime

RUN <<EOF
apt-get update
apt-get install -y libgcc-11-dev curl
rm -rf /var/lib/apt/lists/*
EOF

RUN mkdir /sqnc-node /data

COPY --from=setup /tmp/sqnc-node /sqnc-node/

WORKDIR /sqnc-node

ENTRYPOINT [ "/sqnc-node/sqnc-node" ]

HEALTHCHECK CMD curl -f http://localhost:9944/health || exit 1

EXPOSE 30333 9615 9944

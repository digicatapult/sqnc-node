FROM ubuntu:jammy AS setup

RUN apt-get update && apt-get install -y curl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp/

RUN curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.2/fetch_linux_amd64 --output ./fetch && chmod +x ./fetch

ARG DSCP_VERSION=latest
ARG DSCP_REPO=https://github.com/digicatapult/dscp-node

RUN ./fetch --repo="${DSCP_REPO}" --tag="${DSCP_VERSION}" --release-asset="dscp-node-.*-x86_64-unknown-linux-gnu.tar.gz" ./ \
  && mkdir ./dscp-node \
  && tar -xzf ./dscp-node-*-x86_64-unknown-linux-gnu.tar.gz -C ./dscp-node

FROM ubuntu:jammy AS runtime

RUN apt-get update && apt-get install -y libgcc-11-dev binutils && rm -rf /var/lib/apt/lists/*

RUN mkdir /dscp-node /data

COPY --from=setup /tmp/dscp-node /dscp-node/

WORKDIR /dscp-node

ENTRYPOINT [ "/dscp-node/dscp-node" ]

EXPOSE 30333 9933 9944

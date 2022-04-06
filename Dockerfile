FROM bitnami/minideb:bullseye AS setup

RUN install_packages curl ca-certificates

WORKDIR /tmp/

RUN curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.2/fetch_linux_amd64 --output ./fetch && chmod +x ./fetch

ARG DSCP_VERSION=latest

RUN ./fetch --repo="https://github.com/digicatapult/vitalam-node" --tag="${DSCP_VERSION}" --release-asset="dscp-node-.*-x86_64-unknown-linux-gnu.tar.gz" ./ \
  && mkdir ./dscp-node \
  && tar -xzf ./dscp-node-*-x86_64-unknown-linux-gnu.tar.gz -C ./dscp-node

FROM bitnami/minideb:bullseye AS runtime

RUN install_packages libgcc-10-dev

RUN mkdir /dscp-node /data

COPY --from=setup /tmp/dscp-node /dscp-node/

WORKDIR /dscp-node

ENTRYPOINT [ "/dscp-node/dscp-node" ]

EXPOSE 30333 9933 9944

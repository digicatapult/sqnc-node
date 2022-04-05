FROM bitnami/minideb:bullseye AS setup

RUN install_packages curl ca-certificates

WORKDIR /tmp/

RUN curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.2/fetch_linux_amd64 --output ./fetch && chmod +x ./fetch

ARG VITALAM_VERSION=latest

RUN ./fetch --repo="https://github.com/digicatapult/vitalam-node" --tag="${VITALAM_VERSION}" --release-asset="vitalam-node-.*-x86_64-unknown-linux-gnu.tar.gz" ./ \
  && mkdir ./vitalam-node \
  && ls *.gz |xargs -n1 tar -xzf 

FROM bitnami/minideb:bullseye AS runtime

RUN install_packages libgcc-10-dev

RUN mkdir /vitalam-node /data

COPY --from=setup /tmp/vitalam-node /vitalam-node/

WORKDIR /vitalam-node

ENTRYPOINT [ "/vitalam-node/vitalam-node" ]

EXPOSE 30333 9933 9944

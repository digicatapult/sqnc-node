FROM bitnami/minideb:bullseye AS setup

RUN apt-get update && apt-get install -y \
  curl
  #libgcc
  # libstdc++

RUN mkdir /tmp/vitalam-node/
WORKDIR /tmp/vitalam-node/

RUN curl -L https://github.com/gruntwork-io/fetch/releases/download/v0.4.2/fetch_linux_amd64 --output ./fetch && chmod +x ./fetch

RUN ./fetch --repo="https://github.com/digicatapult/vitalam-node" --tag="v2.5.1" --release-asset="vitalam-node-v2.5.1-x86_64-unknown-linux-gnu.tar.gz" ./ \
  && tar -xzf ./vitalam-node-v2.5.1-x86_64-unknown-linux-gnu.tar.gz 

FROM bitnami/minideb:bullseye AS runtime

RUN apt-get update \
  && apt-get install -y \
  build-essential \
  libgcc-10-dev

RUN mkdir /vitalam-node /data
COPY --from=setup /tmp/vitalam-node /vitalam-node/

WORKDIR /vitalam-node

CMD /vitalam-node/vitalam-node

EXPOSE 30333 9933 9944

# #docker run -it --rm -h node-0 -e IDENTITY=dev -e WS=true -p 30333:30333 -p 9944:9944 -p 9933:9933 vitalam-substrate-node ./run.sh

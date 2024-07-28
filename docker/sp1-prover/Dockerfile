FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

# copy the binaries
# COPY sp1-prover/target/$TARGETARCH/release/prover /usr/local/bin/prover
COPY sp1-prover/target/x86_64-unknown-linux-musl/release/prover /usr/local/bin/prover
RUN chmod +x /usr/local/bin/prover

# When running as a Docker service, we always want a healthcheck endpoint, so set a default for the
# port that the HTTP server will run on. This can be overridden in any given deployment environment.
#ENV PROVER_PORT=80
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:80/healthcheck || exit 1

CMD ["/usr/local/bin/prover"]
# This is the build stage for Axlib. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /axlib
COPY . /axlib
RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Axlib binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Multistage Docker image for Axlib: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="chevdor@gmail.com, devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.description="Axlib is a next-generation framework for blockchain innovation 🚀" \
	io.parity.image.source="https://github.com/paritytech/axia/blob/${VCS_REF}/docker/axlib_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/paritytech/axia/"

COPY --from=builder /axlib/target/release/axlib /usr/local/bin
COPY --from=builder /axlib/target/release/subkey /usr/local/bin
COPY --from=builder /axlib/target/release/node-template /usr/local/bin
COPY --from=builder /axlib/target/release/chain-spec-builder /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /axlib axlib && \
	mkdir -p /data /axlib/.local/share/axlib && \
	chown -R axlib:axlib /data && \
	ln -s /data /axlib/.local/share/axlib && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# Sanity checks
	ldd /usr/local/bin/axlib && \
	/usr/local/bin/axlib --version

USER axlib
EXPOSE 30335 9933 9944 9615
VOLUME ["/data"]

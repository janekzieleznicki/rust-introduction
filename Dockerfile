FROM clearlinux as builder
ARG PROTO_VER=22.2
WORKDIR /usr/src/myapp
COPY . .

RUN swupd bundle-add rust-basic llvm protobuf devpkg-openssl devpkg-protobuf-c wget
RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v${PROTO_VER}/protobuf-${PROTO_VER}.tar.gz && \
    tar -xvzf protobuf-${PROTO_VER}.tar.gz && \
    export PROTOC_INCLUDE=$(pwd)/protobuf-${PROTO_VER}/src/ && \
    cargo install --path app

FROM clearlinux

RUN swupd bundle-add lib-openssl

COPY --from=builder /root/.cargo/bin/server /usr/local/bin/server
COPY --from=builder /root/.cargo/bin/client /usr/local/bin/client
ENV RUST_LOG=debug


ENTRYPOINT ["/usr/local/bin/server"]

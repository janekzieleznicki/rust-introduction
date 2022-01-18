FROM rust:slim-buster as builder-base
WORKDIR /usr/src/myapp
COPY . .
RUN rustup component add rustfmt

FROM builder-base as builder

RUN apt update && \
            apt install -y libssl-dev protobuf-compiler libclang-dev llvm-dev pkg-config clang
RUN cargo install --path app

FROM debian:buster-slim

RUN apt update && \
        apt install -y openssl && \
        rm -rf /var/lib/apt/lists/* && \
    useradd --system --shell /bin/bash --groups sudo server

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

USER server
EXPOSE 50051

ENTRYPOINT ["/usr/local/bin/server"]
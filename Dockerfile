FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl \
  && apt update && apt install -y musl-tools musl-dev libssl-dev pkg-config \
  && update-ca-certificates
WORKDIR /app
COPY ./ .
RUN cargo build --target x86_64-unknown-linux-musl --release --bin cli

FROM scratch
WORKDIR /app
COPY --from=builder /myip/target/x86_64-unknown-linux-musl/release/cli ./
CMD ["/app/cli"]

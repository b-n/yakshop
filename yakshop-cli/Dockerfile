# Builder

FROM rust:1.76-bookworm as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release -p yakshop-cli

# Runner

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/yakshop-cli /usr/local/bin/yakshop-cli

ENTRYPOINT ["yakshop-cli"]

# Builder

FROM rust:1.76-bookworm as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release -p yakshop-web

# Runner

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/yakshop-web /usr/local/bin/yakshop-web

ENTRYPOINT ["yakshop-web"]

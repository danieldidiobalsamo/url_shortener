# syntax=docker/dockerfile:1

FROM rust:latest as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim

# always run with this less-privileged user, and never root
RUN useradd -ms /bin/bash url_shortener_backend
USER url_shortener_backend

COPY --from=builder /app/target/release/url_shortener_backend /usr/local/bin/url_shortener_backend

ENTRYPOINT url_shortener_backend
EXPOSE 8080
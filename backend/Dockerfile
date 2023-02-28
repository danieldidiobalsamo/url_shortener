# syntax=docker/dockerfile:1

FROM rust:latest as builder

RUN mkdir url_shortener_backend
WORKDIR ./url_shortener_backend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

# always run with this less-privileged user, and never root
RUN useradd -ms /bin/bash url_shortener_backend
USER url_shortener_backend

COPY --from=builder /usr/local/cargo/bin/url_shortener_backend /usr/local/bin/url_shortener_backend

ENTRYPOINT url_shortener_backend
EXPOSE 8080
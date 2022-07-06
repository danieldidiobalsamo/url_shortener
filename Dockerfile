# syntax=docker/dockerfile:1

FROM rust:latest as builder

RUN mkdir url_shortener
WORKDIR ./url_shortener
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/url_shortener /usr/local/bin/url_shortener

ENTRYPOINT url_shortener
EXPOSE 8080
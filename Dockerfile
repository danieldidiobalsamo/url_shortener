# syntax=docker/dockerfile:1

FROM rust:latest

RUN mkdir url_shortener
WORKDIR ./url_shortener
COPY . .
RUN cargo install --path .

ENTRYPOINT url_shortener
EXPOSE 8080
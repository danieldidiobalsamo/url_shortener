# syntax=docker/dockerfile:1

# create rust env with cargo chef crate
FROM rust:latest AS chef
WORKDIR /app
RUN cargo install cargo-chef

# generate recipe file to prepare dependencies build
FROM chef AS planner
COPY . /app
RUN cargo chef prepare --recipe-path recipe.json

# build dependencies
FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build app
FROM chef AS builder
COPY . /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# use a very small image to run the app
FROM debian:bookworm-slim
RUN useradd -ms /bin/bash url_shortener_backend # always run with this less-privileged user, and never root
USER url_shortener_backend
COPY --from=builder /app/target/release/url_shortener_backend /usr/local/bin/url_shortener_backend
ENTRYPOINT url_shortener_backend
EXPOSE 8080
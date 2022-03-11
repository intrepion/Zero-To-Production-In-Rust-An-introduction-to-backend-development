# We use the latest Rust stable release as base image
FROM rust:1.59.0

WORKDIR /app

COPY . .

RUN cargo build --release

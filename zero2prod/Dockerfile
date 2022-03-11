FROM rust:1.59.0

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.60.0

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cd zero2prod && cargo build --release
ENTRYPOINT ["./target/release/zero2prod"]

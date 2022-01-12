echo "---- cargo check ----" && \
cargo check && \
echo "---- cargo audit ----" && \
cargo audit --ignore RUSTSEC-2021-0131 --ignore RUSTSEC-2020-0159 --ignore RUSTSEC-2020-0071 && \
echo "---- cargo clippy ----" && \
cargo clippy -- -D warnings && \
echo "---- cargo test ----" && \
cargo test --verbose && \
echo "---- cargo fmt ----" && \
cargo fmt -- --check && \
echo "---- cargo udeps ----" && \
cargo +nightly udeps && \
echo "---- cargo run ----" && \
RUST_LOG=trace cargo run

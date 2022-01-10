cargo check \
&& cargo fmt -- --check \
&& cargo audit --ignore RUSTSEC-2021-0131 --ignore RUSTSEC-2020-0159 --ignore RUSTSEC-2020-0071 \
&& cargo clippy -- -D warnings \
&& cargo test --verbose \
&& cargo run

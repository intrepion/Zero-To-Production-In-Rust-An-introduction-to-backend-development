cargo watch \
-x fmt \
-x check \
-x "audit --ignore RUSTSEC-2021-0131 --ignore RUSTSEC-2020-0159 --ignore RUSTSEC-2020-0071" \
-x "clippy -- -D warnings" \
-x "test -- --nocapture" \
-x "run | bunyan"

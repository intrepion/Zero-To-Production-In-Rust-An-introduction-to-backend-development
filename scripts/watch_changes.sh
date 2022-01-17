cargo watch \
-x fmt \
-x check \
-x "test -- --nocapture" \
-x "run | bunyan"

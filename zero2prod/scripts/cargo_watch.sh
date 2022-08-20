#!/usr/bin/env bash

cargo watch -x fmt -x clippy -x check -x t -x "audit --deny warnings --ignore RUSTSEC-2020-0071 --ignore RUSTSEC-2020-0159 --ignore RUSTSEC-2021-0139" -x "+nightly udeps"

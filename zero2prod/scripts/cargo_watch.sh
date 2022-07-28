#!/usr/bin/env bash

cargo watch -x fmt -x clippy -x check -x t -x "audit --deny warnings --ignore RUSTSEC-2020-0071" -x "+nightly udeps"

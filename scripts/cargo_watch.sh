#!/usr/bin/env bash

cargo watch -x fmt -x clippy -x check -x test -x "audit --ignore RUSTSEC-2020-0071" -x "+nightly udeps"

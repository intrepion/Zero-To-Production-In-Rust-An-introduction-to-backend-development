#!/usr/bin/env bash

cargo watch -x fmt -x clippy -x check -x t -x "audit --deny warnings"

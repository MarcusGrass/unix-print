#!/bin/sh
set -e
cargo test -p unix-print
cargo clippy -p unix-print -- -D warnings
cd no-std-test
cargo clippy
cargo run
#!/bin/bash

set -ex

export RUST_TEST_THREADS=1
export ASAN_OPTIONS="detect_odr_violation=0 detect_leaks=0"
export TSAN_OPTIONS="suppressions=$(pwd)/ci/tsan"

# Run address sanitizer
RUSTFLAGS="-Z sanitizer=address" \
cargo test --target x86_64-unknown-linux-gnu --test test_bytes --test test_buf --test test_buf_mut

# Run thread sanitizer
RUSTFLAGS="-Z sanitizer=thread" \
cargo test --target x86_64-unknown-linux-gnu --test test_bytes --test test_buf --test test_buf_mut

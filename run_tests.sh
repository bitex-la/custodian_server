#!/bin/bash

reset
cargo +nightly test $1 -- --test-threads=1 --nocapture

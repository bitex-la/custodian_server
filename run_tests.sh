#!/bin/bash

reset
cargo test $1 -- --test-threads=1 --nocapture

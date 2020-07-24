#!/usr/bin/env bash

while sleep 1; do ls src/*.rs | entr -cdp bash -c "cargo run < test-input.txt"; done

#!/usr/bin/env bash
set -e

# Modified from https://github.com/gakonst/ethers-rs/blob/master/scripts/examples.sh

ignored=(
    "liquidity",
    "swap",
    "weth"
)

# run all examples
for file in examples/*.rs; do
    name="$(echo "$file" | cut -f 1 -d '.')"
    if [[ "${ignored[*]}" =~ $(basename "$name") ]]; then
        echo "skipping: $file"
        continue
    fi
    echo "running: $file"
    cargo run --example "$(basename "$name")"
done

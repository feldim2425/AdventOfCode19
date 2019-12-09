#!/bin/bash

cargo build --release

TARGET=./target/release

for i in {1..25}
do
    if [ -f "$TARGET/day$i" ]; then
        $TARGET/day$i
    fi
done
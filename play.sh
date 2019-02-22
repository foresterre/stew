#!/bin/bash
rm target/out.png
cargo build
cat resources/botanical.jpg | ./target/debug/brighten -o target/out.png -10 && eog target/out.png


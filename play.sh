#!/bin/bash
rm target/out.png
cargo build
cat resources/botanical.jpg | ./target/debug/blur 5.4 -o target/out.png && eog target/out.png


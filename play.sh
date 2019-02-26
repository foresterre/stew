#!/bin/bash
rm target/out.png
cargo build
# cat resources/botanical.jpg | ./target/debug/crop -o target/out.png 10 10 210 210
# cat resources/botanical.jpg | ./target/debug/crop -f bmp 10 20 180 210 | ./target/debug/blur 15 |  ./target/debug/fliph -o target/out.png
# cat resources/botanical.jpg | ./target/debug/blur 80 |  ./target/debug/brighten -o target/out.png -70
cat resources/botanical.jpg | ./target/debug/unsharpen -f png 40 10 > target/out.png
eog target/out.png


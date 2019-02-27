Stew is very much experimental and work in progress.

# Stew

Stew is a set of image transformation tools, adapted from [sic](https://github.com/foresterre/sic).
Sic specifically has a single executable whereas Stew has a separate executable for each image operation.
This allows the tools in the Stew toolset to be piped to each other and interchangeably with other
cli applications (assuming the output of such tool is supported as input format by Stew).


<br>

_crates.io: [Stew](https://crates.io/crates/stew)_

# Install

With [cargo](https://crates.io/crates/stew) install: `cargo install --force stew`

Pre build binary: see [releases](https://github.com/foresterre/stew/releases)

From the source of this repo:
- Setup rust and cargo with (for example) [rustup](https://rustup.rs/) <br> 
  _Rust version >= 1.31 with 'Rust edition 2018' is required._
- Clone this repo: `git clone https://github.com/foresterre/stew.git`
- Switch to this repo: `cd stew`
- Build a release: `cargo build --release`


# Usage

These example usages assume the `input` image is `botanical.jpg` and the `output` image is `out.png`.

```
crop -i botanical.jpg -o out.png 10 10 210 210
```

```
cat botanical.jpg | crop 10 20 180 210 | blur 15 |  fliph -o target/out.png
```

```
cat resources/botanical.jpg | ./target/debug/blur 80 |  ./target/debug/brighten -o target/out.png -70
```

__Note: when `stdout` is used for the image output, extensions can't be used to determine the image output format.
All Stew tools default to BMP as output format, but you can specify a different option using the `--output-format`
 (shorthand `-f`) cli option, as demonstrated below:__
```
cat resources/botanical.jpg | ./target/debug/unsharpen -f png 40 10 > target/out.png
```

Show usage of individual tools by running them with the help flag: `<tool> --help`, for example `huerotate --help`.

# Available tools:

* [x] `blur` `[u32]`
* [x] `brighten` `[i32]`
* [x] `contrast` `[f32]`
* [x] `convert`
* [x] `crop` `[u32] [u32] [u32] [u32]`
* [x] `filter3x3` `[f32] [f32] [f32] [f32] [f32] [f32] [f32] [f32] [f32]`
* [x] `fliph`
* [x] `flipv`
* [x] `grayscale`
* [x] `huerotate` `[i32]`
* [x] `invert`
* [x] `resize` `[u32] [u32]`
* [x] `rotate90`
* [x] `rotate180`
* [x] `rotate270`
* [x] `unsharpen` `[f32] [i32]`


# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

:guitar: :trumpet: :violin: :saxophone:

## Rust Flip
A coin flipping CLI tool written in Rust using clap.

## Installation
Clone from git `git clone https://github.com/zack37/rust-flip.git`
Build `cargo build --release`
Run `./target/release/rust-flip`

## Usage
```
FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --heads <heads>    Value for heads [default: heads]
    -t, --tails <tails>    Value for tails [default: tails]
    -n, --times <times>    Number of times to flip the coin [default: 1]
```
# SP1 Precompile Test

This repository contains a simple SP1 precompile test program that demonstrates how to use the SP1 precompile feature in a Rust-based project. The program is designed to be executed with or without precompiles, allowing you to compare the performance of both methods.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

## Running the Project

### Build the Program

The program is automatically built through `script/build.rs` when the script is built.

### Execute the Program

To run the program without generating a proof:

```sh
cd script

# without precompile
cargo run --release -- --execute --rounds=1000 --message="Hello, world"

# with precompile
cargo run --release -- --execute --rounds=1000 --message="Hello, world" --use-precompile
```

This will execute the program and display the output.

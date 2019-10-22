# TermUtil

A library offering simple cross-platform functions for simple terminal tasks.

## Compiling

The library requires [Rust](https://rustup.rs/) to be installed in order to
compile.

1. Clone this repository.
2. Target your desired OS:
  - To target Windows 64-bit:
    `rustup run stable-x86_64-msvc cargo build --release`
  - To target Windows 32-bit:
    `rustup run stable-i686-msvc cargo build --release`
  - To target *nix 64-bit:
    `rustup run stable-x86_64-linux-gnu cargo build --release`
  - To target *nix 32-bit:
    `rustup run stable-i686-linux-gnu cargo build --release`
3. Include the header `termutil.h` from the root of this repository.
4. Link to the library once compiled in `target/release`.

## License

This project is released under the BSD 3-Clause License. See `LICENSE` for the
full license content.

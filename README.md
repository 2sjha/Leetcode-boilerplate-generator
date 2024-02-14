# Leetcode Boilerplate Generator

Ever had to debug Leetcode problems? Debugging a Leetcode problem on their website is behind a paywall. And running & debugging a Leetcode problem locally requires you to write the driver code that will call the problem function with example inputs & outputs, `lc-boilerplate-generator` can do just that for you.
This can generate compilable driver code in *C++* and *Rust* with example inputs and assertions against their expected outputs. If the problem inputs and/or outputs are primitive data types or 1-D or 2-D arrays, then this can generate driver code for it.

*Not Supported:*

  - Problems based on dynamic data structures like Linked List, Tree are not supported
  - Problems based on writing a class for a custom data structure.

Generated boilerplate code for Rust
![Demo-Rust](./assets/demo-rust.png)

Generated boilerplate code for C++
![Demo-C++](./assets/demo-cpp.png)


## How to Use

- `./lc-boilerplate-generator <leetcode-problem-url> <language>` (supported languages: `cpp`, `rust`, `java`, `python3`)

Or

- `./lc-boilerplate-generator` and it will ask for both these input.

## Setup

- Install Rust from [https://rustup.rs/](https://rustup.rs/)
- Run with `cargo run` or Build binary with `cargo build --release`

## TODO

- [ ] Support Java and Python with full driver code
- [ ] Handle problems where return type is void & the problem checks the updated version of the input, e.g. [this LC Problem](https://leetcode.com/problems/rotate-array)

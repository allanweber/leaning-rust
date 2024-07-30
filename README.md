# Learning Rust

Rust book URL

[https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/title-page.html)

## Installation

### WSL

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc --version
```

## Cargo

[https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)

* We can create a project using ```cargo new```
* We can build a project using ```cargo build```
* We can build and run a project in one step using ```cargo run```
* We can build a project without producing a binary to check for errors using ```cargo check```
* When your project is finally ready for release, you can use ```cargo build --release``` to compile it with optimizations

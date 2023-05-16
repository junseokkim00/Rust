# Tutorial for Rust

Reference: [Link](https://google.github.io/comprehensive-rust/)

## Index
1. Day 1: Basic Rust, ownership and borrow checker
2. Day 2: Compound data types, pattern matching, standard library
3. Day 3: Traits and generics, error handling, testing, unsafe Rust

## Download Rust

Rustup
``` shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* update rustup
```
rustup update
```
* uninstall rust and `rustup`
```
$ rustup self uninstall
```
* look up rust documentation
```
$ rustup doc
```

Package Managers
``` shell
$ sudo apt install cargo rust-src rustfmt
```


## How to use Cargo

* create new project with Cargo
```
$ cargo new "Project name"
```
* run project
```
$ cd "Project name"
$ cargo run
```

* check errors
```
$ cargo check
```
* compile only
```
$ cargo build
```
### configuration of the project
* `src`: src code for the project
* `target`: ???
* `.gitignore`: we all know
* `Cargo.lock`: ???
* `Cargo.toml`: dependencies of the project

## Why Rust

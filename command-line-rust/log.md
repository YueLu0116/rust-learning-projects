# Reading Notes for _Command-Line Rust_

## Preface

**About Rust**

> Rust has a fairly steep learning curve
>
> “Rust is described as a systems programming language that has been designed for performance and safety.”
>
> “Rust is not an object-oriented (OO) language like Java as there are no classes or inheritance in Rust. Instead Rust uses a struct (structure) to represent complex data types and traits to describe how types can behave.”

## Chapter 1

**Writing integration tests for Rust**

The organization of a rust project:

```
hello
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   ├── rls
│   └── tmp
└── tests
    └── cli.rs
```

Use crate `assert_cmd` 

> “I can use the crate assert_cmd to find the program in my crate directory. I first need to add this as **a development dependency** to Cargo.toml. This tells Cargo that I only need this crate for testing and benchmarking:”
>
> [package]
> name = "hello"
> version = "0.1.0"
> edition = "2018"
>
> [dependencies]
>
> [dev-dependencies]
> assert_cmd = "1"
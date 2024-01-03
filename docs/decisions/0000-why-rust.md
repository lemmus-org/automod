# Why Rust?

## Context and Problem Statement

Which language should be chosen as a basis for this project?

## Considered Options

* Rust
* Python

## Decision Outcome

Chosen option: "Rust", because:
* Lemmy itself is written in Rust and publishes a [crate](https://crates.io/crates/lemmy_api_common) for API data types. This allows us to easily reuse that code without having to reimplement and simplifies version upgrades.
* It minimizes the resource usage thereby reducing cost and increasing deployment options.
* Memory safety and type system are an added benefit!

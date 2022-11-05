# Rust Template: Web service in 2022

Simple web service template that tries to follow best practices in terms of cli, crates,
patterns, configuration and file structure.

If you spot anti-patterns, bad practices, outdated crates, missing features or anything really: feel free to open an issue.

## CLI

Run `cargo run -- help` to learn more.

## xtask

Shell tasks following the [xtask pattern](https://github.com/matklad/cargo-xtask).

Run `cargo xtask help` to learn more.

## Template crates

- CLI: `clap`
- Webserver / HTTP: `axum` / `hyper`
- Error handling: `error-stack` / `thiserror`
- Async runtime: `tokio`
- Tracing: `tracing` / `tracing-futures` / `tracing-subscriber`
- De/serialization: `serde` / `serde-with`

## Template structure

```shell
src/
  cli.rs      # CLI parsing and commands
  config.rs   # Configuration handling
  error.rs    # Error types
  http.rs     # Webserver / HTTP handling
  lib.rs      # WebService struct
  main.rs     # Binary
  state.rs    # Shared application state
xtask/
  main.rs     # xtasks
```

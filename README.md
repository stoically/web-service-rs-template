# Rust Template: Web service in 2022

Simple web service template that tries to follow best practices in terms of cli, crates,
patterns, configuration and file structure. The template wants to be as lean as possible, meaning that it does only the most basic wiring up of popular crates together with a container building xtask – something that tends to eat the first few hours of that New Shiny Side Project.

If you spot anti-patterns, bad practices, outdated crates, missing features or anything really: feel free to open an issue.

## CLI

**Features**

- Spawn web service
- Generate configuration file

Run `cargo run -- help` to learn more.

## xtask

**Features**

- Build docker container

Shell tasks following the [xtask pattern](https://github.com/matklad/cargo-xtask).

Run `cargo xtask help` to learn more.

## Quick start

1. `cargo run -- config generate` and adjust the `database.uri` in the generated `config.yaml`
2. `cargo run -- spawn`
3. Visit `http://localhost:10100`

## Template crates

- CLI: `clap`
- Webserver / HTTP: `axum` / `hyper`
- Database / Postgres: `sqlx`
- Configuration: `sample-config`
- Error handling: `error-stack` / `thiserror`
- Async runtime: `tokio`
- Tracing: `tracing` / `tracing-futures` / `tracing-subscriber`
- De/serialization: `serde` / `serde-with`

## Template structure

```shell
src/
  cli.rs      # CLI parsing and commands
  database.rs # Database handling
  config.rs   # Configuration handling
  error.rs    # Error types
  http.rs     # Webserver / HTTP handling
  lib.rs      # WebService struct
  main.rs     # Binary
  state.rs    # Shared state
xtask/
  main.rs     # xtasks
```

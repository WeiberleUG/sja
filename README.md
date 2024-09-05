[![Build Rust project](https://github.com/Vaengir-Projects/rigit/actions/workflows/compile.yml/badge.svg)](https://github.com/Vaengir-Projects/rigit/actions/workflows/compile.yml)
[![Dependency status](https://deps.rs/repo/github/Vaengir-Projects/rigit/status.svg)](https://deps.rs/repo/github/Vaengir-Projects/rigit)

# SJA Webapp

This is a webapp written in rust using [leptos](https://crates.io/crates/leptos) and [axum](https://crates.io/crates/axum).

## Dependencies to work with this repository

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `cargo install cargo-leptos --locked` - install `cargo-leptos` binary
2. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` -

## Initial setup

1. `rustup override set nightly` - run project in nightly

## Running your project

```bash
cargo leptos watch
```

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
sja
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="sja"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This project is released under the [MIT-License](https://github.com/Weiberle17/sja/blob/main/LICENSE).

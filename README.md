# VTT Player Stats

A small Rust prototype for tracking player statistics for a virtual tabletop (VTT).

The current version demonstrates a basic kill counter: it starts at zero, adds one kill with `add_kill`, and prints the updated count. It does not yet accept player input or save statistics between runs.

## Requirements

- Rust 1.85 or later with Cargo (the project uses Rust edition 2024).
- No external crate dependencies are required.

## Run locally

Clone the repository and run the program:

```sh
git clone https://github.com/csbabcock/vtt-player-stats.git
cd vtt-player-stats
cargo run
```

Expected program output:

```text
Kills: 1
```

## Build

```sh
cargo build --release
```

The compiled executable is written to `target/release/`.

## Project structure

- `src/main.rs` — the kill-count helper and executable entry point.
- `Cargo.toml` — package metadata and dependencies.
- `Cargo.lock` — Cargo's dependency lockfile.

Build artifacts in `target/` and IDE settings in `.idea/` are excluded from version control.

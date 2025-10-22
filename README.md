# JSON IDL Code Generator

This Rust application reads a JSON file and a Jinja template, then generates a text file using the minijinja template engine.

## Usage

```
cargo run -- <template.jinja> <input.json> <output.txt>
```

- `<template.jinja>`: Path to your Jinja template file
- `<input.json>`: Path to your input JSON file
- `<output.txt>`: Path to the output file to generate

## Dev Container

A `.devcontainer` is provided with Rust, rust-analyzer, and development tools pre-installed. Open this folder in VS Code and reopen in the container to get started.

## Development
- Rust dependencies: `minijinja`, `serde`, `serde_json`
- VS Code extension: `rust-analyzer` (auto-installed in devcontainer)

## Tasks
- Build: `cargo build`
- Run: `cargo run -- <template.jinja> <input.json> <output.txt>`

## License
MIT

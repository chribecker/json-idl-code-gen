# JSON IDL Code Generator

This Rust application reads a JSON file and a Jinja templates, then generates a text file using the minijinja template engine.

## Usage

Generate code using custom templates.
```
json-idl-code-gen -t <templates> -i <input> -o <output>
```

Generate code using internal templates.
```
json-idl-code-gen -i <input> -o <output>
```

- `<template>`: Folder path to your Jinja template files and config.yaml
- `<input>`: File Path to your input JSON or YAML file
- `<output>`: Folder path to the output folder to generate

## Dev Container

A `.devcontainer` is provided with Rust, rust-analyzer, and development tools pre-installed. Open this folder in VS Code and reopen in the container to get started.

## Development
- Rust dependencies: `minijinja`, `serde`, `serde_json`
- VS Code extension: `rust-analyzer` (auto-installed in devcontainer)

## Tasks
- Build: `cargo build`
- Run: `cargo run -- -t <templates> -i <input> -o <output>`

## License
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)

# Copilot Instructions for json-idl-code-gen

## Project Overview
This project is named `json-idl-code-gen`. 
It is focused on generating code from JSON-based Interface Definition Language (IDL) specifications.
The project is set up in a dev container running Ubuntu 24.04.3 LTS with Rust 1.83.0 installed.
The build system used is Bazel with rust_rules for Rust integration.
The project shall be deployed and built using Bazel.

The deployment targets are
- an executable command-line tool written in Rust.
- a bazel rule that uses the executable command-line tool written in Rust and wraps it as a bazel rule.

The command-line tool reads JSON or YAML IDL files and generates Rust and Cpp code based on templates using the minijinja templating engine.
The tool also embeds static files using the rust-embed crate.
The tool has the following command-line interface:

`comgen --input <input_file> --output <output_dir>`

or with the optional template folder argument:

`comgen --input <input_file> --output <output_dir> --template <template_dir>`

Where:
- `<input_file>` is the path to the JSON or YAML IDL file.
- `<output_dir>` is the directory where the generated code will be placed.
- `<template_dir>` is an optional directory containing custom templates for code generation.

The bazel rule which wraps the command-line tool is named `comgen_source`.

The bazel rule `comgen_source` can be used as follows in a customer BUILD.bazel file:

```
comgen_source(
    name = "my_generated_code",
    src = "path/to/idl_file.json",
    out_dir = "path/to/output/dir",
)
```

Where:
- `src` is the path to the JSON or YAML IDL file and maps to the `--input` argument of the command-line tool.
- `out_dir` is the directory where the generated code will be placed and maps to the `--output` argument of the command-line tool.

The following dependencies and versions are used in this project:
- Rust edition 2021
- Bazel build system version 8.3.0
- rust_rules version 0.66.0
- minijinja version 2.12.0
- serde version 1.0.288
- rust-embed version 8.8.0 with feature "debug-embed"

## Coding Guidelines

- Prefer Rust for all code generation tools, scripts, and utilities.
- Follow idiomatic style for the chosen language (rustfmt for Rust).
- Use strong typing in Rust.
- Write modular, testable code. Place tests in a `tests/` directory.
- Document all public functions and modules with docstrings or Rust doc comments.
- Use relative imports for internal modules.
- For command-line tools, use `clap` (Rust).
- Prefer standard library and well-known, maintained dependencies.
- For JSON parsing, use `serde_json` (Rust).
- For YAML parsing, use `serde_yaml` (Rust).
- Use `minijinja` for templating in Rust.
- Use `rust-embed` for embedding static files in Rust.
- Write unit tests for all functions and integration tests for modules.
- Follow semantic versioning for releases.
- Handle errors gracefully using `Result` and `Option` types in Rust.

## Project Structure

- Place Rust source code in a `src/` directory or as a Bazel Cargo crate.
- Place code generation templates in a `templates/` directory.
- Place example IDL files in an `examples/` directory.
- Place documentation in a `docs/` directory.

## Commit and Version Control

- Use Git for version control.
- Write clear, concise commit messages.
- Track large files with Git LFS if needed.


# JSON IDL Code Generator

This Rust application reads a JSON file and a Jinja templates, then generates a text file using the minijinja template engine.

## Usage

Generate code using custom templates.
```
comgensource -t <templates> -i <input> -o <output>
```

Generate code using internal templates.
```
comgensource -i <input> -o <output>
```

- `<template>`: Folder path to your Jinja template files and config.yaml
- `<input>`: File Path to your input JSON or YAML file
- `<output>`: Folder path to the output folder to generate

## Dev Container

A `.devcontainer` is provided with Rust, rust-analyzer, and development tools pre-installed. Open this folder in VS Code and reopen in the container to get started.

## Development
- Rust dependencies: `minijinja`, `serde`, `serde_json`
- VS Code extension: `rust-analyzer` (auto-installed in devcontainer)

## Bazel Build and Run
- Build Comgen: `bazel build //comgen:comgen_source`
- Test All: `bazel test //...`
- Run: `bazel run //comgen:comgen_source -- -i $PWD/<input> -o $PWD/<output>`

## Run Tool after build
`./bazel-bin/comgensource -i <input> -o <output>`

or with example files:
`./bazel-bin/comgensource -i ./tests/car_window_types.yaml -o testoutput`

## License
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)

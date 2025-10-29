# JSON IDL Code Generator

This Rust application reads a JSON file and a Jinja templates, then generates a text file using the minijinja template engine.

## Transformation Rules for Template Engine
- [Standard Types Mapping](doc/StandardDataTypeMapping.MD)
- [C++ Types Generation](doc/Cpp_Types_Gen.MD)
- [C++ Interface Generation](doc/Cpp_Interface_Gen.MD)
- [C++ Interface Skeleton Generation](doc/Cpp_Skeleton_Gen.MD)
- [Rust Types Generation](doc/Rust_Types_Gen.MD)
- [Rust Interface Skeleton Generation](doc/Rust_Interface_Gen.MD)

## Technical Challenges
- Jinja template rendering with complex data structures
  - Modifying context data for complex data structures is not directly supported by MiniJinja
- Error handling for file I/O and template rendering
- Bazel build integration
  - Hermetic builds and tests does not have write access to the local file system
  - Dependency management only to files and not to folders.<br>
    `cc_binary` and `cc_library` can not access files in generated folders.

## Open Points
- Support for additional data types and structures
- Enhanced error reporting and logging
- Topological sorting of data types for C++ generation
- Namespace handling in Rust and C++
- File separation of types and interfaces

## Dev Container

A `.devcontainer` is provided with Rust, rust-analyzer, and development tools pre-installed. Open this folder in VS Code and reopen in the container to get started.

## Development
- Rust dependencies: `minijinja`, `serde`, `serde_json`
- VS Code extension: `rust-analyzer` (auto-installed in devcontainer)

## Bazel Build and Run
- Build Comgen: `bazel build //:comgen`
- Test All: `bazel test //...`
- Run: `bazel run //:comgen -- -i $PWD/<input> -o $PWD/<output>`

## Tool Usage

Generate code using custom templates.
```
comgen -t <templates> -i <input> -o <output>
```

Generate code using internal templates.
```
comgen -i <input> -o <output>
```

- `<input>`: File Path to your input JSON or YAML file
- `<output>`: Folder path to the output folder to generate
- `<template>`: Optional folder path to your Jinja template files and config.yaml
- `<namespace>`: Optional filter that only generates interfaces for the specified namespace

```
> comgen --help
mw::com code generator from JSON/YAML input

Usage: comgen [OPTIONS] --input <FILE> --output <FOLDER>

Options:
  -n, --namespace <NAMESPACE>  (Optional) Namespace filter (only generate interfaces for this namespace)
  -t, --templates <TEMPLATES>  (Optional) Folder containing Jinja templates
  -i, --input <FILE>           Input file (JSON, YAML, or YML)
  -o, --output <FOLDER>        Output folder for generated files
  -v, --verbose                (Optional) Verbose output flag
  -h, --help                   Print help
  -V, --version                Print version
```

## Run Tool after build
`./bazel-bin/comgen -i <input> -o <output>`

or with example files:

`./bazel-bin/comgen -i ./tests/car_window_types.yaml -o testoutput`

## License
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)

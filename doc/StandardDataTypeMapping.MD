# Data Type Mapping

This document describes the IDL data type mappings used in the code generation process for different programming languages Rust and C++. It outlines how various data types are represented in each target language, ensuring consistency and compatibility across different implementations.

## Table of Primitive Types and Strings:
| IDL Data Type   | C++                   | Rust         |
|-----------------|-----------------------|--------------|
| int8            | `std::int8_t`         | `i8`         |
| uint8           | `std::uint8_t`        | `u8`         |
| int16           | `std::int16_t`        | `i16`        |
| uint16          | `std::uint16_t`       | `u16`        |
| int32           | `std::int32_t`        | `i32`        |
| uint32          | `std::uint32_t`       | `u32`        |
| int64           | `std::int64_t`        | `i64`        |
| uint64          | `std::uint64_t`       | `u64`        |
| float           | `std::float_t`        | `f32`        |
| double          | `std::double_t`       | `f64`        |
| bool            | `bool`                | `bool`       |
| string          | `std::string`         | `string`     |
| char            | `char`                | `char`       |
| byte            | `std::byte`           | `u8`         |
| size_t          | `std::size_t`         | `usize`      |


## Compound Types

| IDL Data Type   | C++                   | Rust            |
|-----------------|-----------------------|-----------------|
| enum            | `enum class`          | `enum`          |
| struct          | `struct`              | `struct`        |
| array<T,N>      | `std::array<T, N>`    | `[T; N]`        |


## Additional Compound Types may be not supported in S-Core, but here is a suggested mapping:

| IDL Data Type   | C++                   | Rust            |
|-----------------|-----------------------|-----------------|
| vector<T>       | `std::vector<T>`      | `Vec<T>`        |
| map<K, V>       | `std::map<K, V>`      | `HashMap<K, V>` |
| optional<T>     | `std::optional<T>`    | `Option<T>`     |
| variant<Ts...>  | `std::variant<Ts...>` | `enum`          |
| tuple<Ts...>    | `std::tuple<Ts...>`   | `(Ts...)`       |



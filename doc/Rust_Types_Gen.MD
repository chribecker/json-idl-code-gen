# Generate Types for Rust

Data type mapping for Rust is described in [Standard Data Type Mapping](./StandardDataTypeMapping.MD).
In contrast to C++, the order of type definitions in Rust is handled automatically by the Rust compiler.

## Namespace in Rust

**Parameters:**
- `Namespace`: The name of the namespace to be created.

In rust the source file name represents the namespace `$Namespace.rs`.
- Second level of namespaces are represented as modules within the file.
  ```rust
  pub mod $sub_namespace {
    // Sub types defined here
  }
  ```

## Required Imports and Modules in Rust

The Default trait from the standard library is required when generating structs with default values.

    ```rust
    use std::default::Default;
    ```

## Enums in Rust

**Parameters:**
- `EnumName`: The name of the enum to be created.
- `RustType`: The underlying Rust type for the enum representation (e.g., `u8`, `i32`).
- A list of enum literals and their associated values:
  - `EnumLiteralN`: The N-th literal name of the enum.
  - `EnumValueN`: The value associated with the N-th literal.

- In the file $Namespace.rs, the enum is defined as follows:
    ```rust
    #[repr($RustType)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum $EnumName {
      $EnumLiteral1 = $EnumValue1,
      $EnumLiteral2 = $EnumValue2,
    }
    ```

- The default value for the enum is the first literal defined.
  If the default value is not the first literal, you can specify it using the `DefaultEnumLiteral` parameter.
    ```rust
    impl Default for $EnumName {
      fn default() -> Self {
        $EnumName::$DefaultEnumLiteral
      }
    }
    ```

## Structs in Rust

**Parameters:**
- `StructName`: The name of the struct to be created.
- A list of fields with their associated types:
  - `FieldNameN`: The N-th field name of the struct.
  - `FieldTypeN`: The type associated with the N-th field.
  - `FieldDefaultN` (optional): The default value for the N-th field.
- In the file $Namespace.rs, the struct is defined as follows:
    ```rust
    #[derive(Debug, Clone, PartialEq)]
    pub struct $StructName {
      pub $FieldName1: $FieldType1,
      pub $FieldName2: $FieldType2,
    }
    ```
- If default values are provided for fields, implement the Default trait as follows:
    ```rust
    impl Default for $StructName {
      fn default() -> Self {
        $StructName {
          $FieldName1: $FieldDefault1,
          $FieldName2: $FieldDefault2,
        }
      }
    }
    ``` 

## Arrays in Rust

**Parameters:**
- `ArrayTypeName`: The name of the array type to be created.
- `ElementType`: The type of the elements in the array.
- `Size`: The size of the array.
- In the file $Namespace.rs, the array type is defined as follows:
    ```rust
    pub type $ArrayTypeName = [$ElementType; $Size];
    ```
-

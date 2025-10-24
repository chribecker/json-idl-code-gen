# Generate Types for C++

**See also:** [Standard Data Type Mapping](./StandardDataTypeMapping.MD).

Data types in C++ must be generated in a topological order, meaning that if a type `A` depends on type `B`, then type `B` must be defined before type `A`.

## Namespace in C++

**Parameter:**
- `Namespace`: The name of the namespace to be created.

- In C++, the namespace is represented using the `namespace` keyword.
  ```cpp
  namespace $Namespace {
     // Types defined here
  }
  ```
- Nested namespaces are represented using nested `namespace` blocks.
  ```cpp
  namespace $Namespace {
    namespace $sub_namespace {
       // Types defined here
    }
  }
  ```

## Include Guards

```cpp
#ifndef $NAMESPACE_H
#define $NAMESPACE_H

// Types defined here

#endif // $NAMESPACE_H
```

## Required Includes in C++

```cpp
#include "score/mw/com/types.h"
```


## Enums in C++

**Parameters:**
- `EnumName`: The name of the enum to be created.
- `CppType`: The underlying Rust type for the enum representation (e.g., `std::uint8_t`, `std::int32_t`).
- A list of enum literals and their associated values:
  - `EnumLiteralN`: The N-th literal name of the enum.
  - `EnumValueN`: The value associated with the N-th literal.
    - if the data type is unsigned, the value should be suffixed with 'U' (e.g., `1U`, `2U`).

- In the file $Namespace.cpp, the enum is defined as follows:
    ```cpp
    enum class WindowCommand : $CppType
    {
      $EnumLiteral1 = $EnumValue1,
      $EnumLiteralN = $EnumValueN,
    };
    ```
- If a default value is provided for a literal then that literal should be used during variable initialization.
- If no default value is provided, the first literal is used as the default.

## Structs in C++

**Parameters:**
- `StructName`: The name of the struct to be created.
- A list of fields with their associated types:
  - `FieldNameN`: The N-th field name of the struct.
  - `FieldTypeN`: The type associated with the N-th field.
  - `FieldDefaultN` (optional): The default value for the N-th field.

- In the file $Namespace.cpp, the struct is defined as follows:
    ```cpp
    struct $StructName
    {
      $FieldType1 $FieldName1{$FieldDefault1};
      $FieldTypeN $FieldNameN{$FieldDefaultN};
    };
    ```

- If a struct is directly used as a data type for an event then default constructors should be provided that initializes all fields to their default values.
    ```cpp
    $StructName() $FieldName1(FieldDefault1), $FieldNameN(FieldDefaultN) = default;
    $StructName($StructName&&) = default;
    $StructName(const $StructName&) = default;
    $StructName& operator=($StructName&&) = default;
    $StructName& operator=(const $StructName&) = default;
    ```

### Arrays in C++

**Parameters:**
- `ArrayName`: The name of the array type to be created.
- `ElementType`: The type of elements in the array.
- `Size`: The size of the array.
- `ArrayDefault` (optional): The default value for the array elements.
  - Other than zero-initialization, the default value should be provided as a comma-separated list enclosed in braces (e.g., `1, 2, 3`).
  - A full default initialization with other than zero values is only supported by a class/struct constructor.

- In the file $Namespace.cpp, the array type is defined as follows:
    ```cpp
    using $ArrayName = std::array<$ElementType, $Size> = {$ArrayDefault};
    ```


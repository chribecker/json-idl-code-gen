# C++ Skeleton Generation

For each interface defined in the IDL, a corresponding Rust skeleton needs to be created to handle the communication between the C++, Rust and the S-CORE MW components.
Below is an example of how to define these skeletons using the provided macros.

### Interface Definition

Each interface requires the following parameters:
- `UID`: The unique identifier for the interface.
- `InterfaceName`: The name of the interface.
  - e.g. `IpcBridgeInfo`
- `Proxy`: The proxy class associated with the interface.
  - fully qualified cpp interface type name with namespace and suffix `Proxy`
  - e.g. `::car_window_types::IpcBridgeInfoProxy`
- `Skeleton`: The skeleton class associated with the interface.
  - fully qualified cpp interface type name with namespace and suffix `Skeleton`
  - e.g. `::car_window_types::IpcBridgeInfoSkeleton`
- A list of events with their associated types:
  - `UID`: The unique identifier for the interface (The same UID as the interface).
    - a concatenation of the namespace and interface name with underscores and suffix `_interface`
    - e.g. `car_window_types_WindowInfo_interface`
  - `CppEventType`: The type associated with the event.
    - fully qualified cpp type name with namespace
    - e.g. `::car_window_types::WindowInfo`
  - `EventName`: The name of the event variable.
    - lowercase event name with underscores
    - e.g. `window_info_`


### UID Generation
The `UID` for the interface must be unique across all interfaces in the system.
The name can be constructed by combining the namespace and interface name with underscores.
- `$UID = $Namespace_$InterfaceName_interface`

e.g.
  - Namespace: `car_window_types`
  - Interface Name: `WindowInfo`
  - results into: `car_window_types_WindowInfo_interface`

### Interface Code Generation

```cpp
BEGIN_EXPORT_MW_COM_INTERFACE($UID, $Proxy, $Skeleton)
EXPORT_MW_COM_EVENT($UID, $CppEventType1, $EventName1)
EXPORT_MW_COM_EVENT($UID, $CppEventTypeN, $EventNameN)
END_EXPORT_MW_COM_INTERFACE()
```

### Data Type Code Generation

Each data type requires the following parameters:
- `UID`: The name of the data type to be registered.
  - fully qualified cpp type name with namespace concat with `_` and suffix `_type`
  - e.g. `car_window_types_WindowInfo_type`
  - The `UID` for the data type must be unique across all interfaces in the system.
- `CppDataTypeName`: The name of the data type to be registered.
  - fully qualified cpp type name with namespace
  - e.g. `::car_window_types::WindowInfo`

```cpp
EXPORT_MW_COM_TYPE($UID, $CppDataTypeName)
```


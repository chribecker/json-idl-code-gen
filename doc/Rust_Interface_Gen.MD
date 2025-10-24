
## Rust Interface Skeletons

### Interface Definition

Each interface requires the following parameters:
- `UID`: The unique identifier for the interface.
- `InterfaceName`: The name of the interface.
  - e.g. `IpcBridgeInfo`
- A list of events with their associated types:
  - `RustEventType`: The type associated with the event.
    - fully qualified rust type name with namespace
    - e.g. `create::WindowInfo`
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
```rust
mw_com::import_interface!($UID, $InterfaceName, {
    $EventName: Event<$RustEventType1>,
});
```


### Data Type Code Generation

Each data type requires the following parameters:
- `UID`: The name of the data type to be registered.
  - fully qualified rust type name with namespace concat with `_` and suffix `_type`
  - e.g. `car_window_types_WindowInfo_type`
  - The `UID` for the data type must be unique across all interfaces in the system.
- `RustDataTypeName`: The name of the data type to be registered.
  - fully qualified rust type name with crate
  - e.g. `crate::WindowInfo`


```rust
mw_com::import_type!($UID, $RustDataTypeName);
```

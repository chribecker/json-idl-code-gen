
# C++ Interface Skeletons


```cpp
template <typename Trait>
class IpcBridgeInfoInterface : public Trait::Base
{
  public:
    using Trait::Base::Base;

    typename Trait::template Event<WindowInfo> window_info_{*this, "window_info"};
};

template <typename Trait>
class IpcBridgeControlInterface : public Trait::Base
{
  public:
    using Trait::Base::Base;

    typename Trait::template Event<WindowControl> window_control_{*this, "window_control"};
};

using IpcBridgeInfoProxy = score::mw::com::AsProxy<IpcBridgeInfoInterface>;
using IpcBridgeInfoSkeleton = score::mw::com::AsSkeleton<IpcBridgeInfoInterface>;
using IpcBridgeControlProxy = score::mw::com::AsProxy<IpcBridgeControlInterface>;
using IpcBridgeControlSkeleton = score::mw::com::AsSkeleton<IpcBridgeControlInterface>;
```

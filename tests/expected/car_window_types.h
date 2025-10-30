/********************************************************************************
 * Copyright (c) 2025 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/
#ifndef SCORE_CAR_WINDOW_TYPES_H
#define SCORE_CAR_WINDOW_TYPES_H

#include "score/mw/com/types.h"

namespace car_window_types {
    /*
     * Enumerator that specifies the possible commands to control the car window
     */
    enum class WindowCommand : std::uint32_t {
        Stop = 0U,
        Open = 1U,
        Close = 2U,
        Exit = 3U,
    };

    /*
     * Enumerator that specifies the possible states of the car window
     */
    enum class WindowState : std::uint32_t {
        Stopped = 0U,
        Opening = 1U,
        Closing = 2U,
        Open = 3U,
        Closed = 4U,
        Shutdown = 5U,
    };

    /*
     * Control command structure for the car window
     */
    struct WindowControl {
        WindowControl(): command(WindowCommand::Stop) {};
        WindowControl(WindowControl&&) = default;
        WindowControl(const WindowControl&) = default;
        WindowControl& operator=(WindowControl&&) = default;
        WindowControl& operator=(const WindowControl&) = default;
 
        // Command to control the window
        WindowCommand command;
    };

    /*
     * Status and information structure of the car window
     */
    struct WindowInfo {
        WindowInfo(): state(WindowState::Stopped), pos(0), lastpos{ 0 } {};
        WindowInfo(WindowInfo&&) = default;
        WindowInfo(const WindowInfo&) = default;
        WindowInfo& operator=(WindowInfo&&) = default;
        WindowInfo& operator=(const WindowInfo&) = default;
 
        // Current state of the window
        WindowState state;
 
        // Position of the window (0% = closed, 100% = open)
        std::uint32_t pos;
 
        // Last 5 positions of the window
        std::uint32_t lastpos[5];
    };


    /*
     * Type definition of uint16 as MyU16
     */
    typedef std::uint16_t MyU16;

    /*
     * Type definition of an array of 10 uint16
     */
    typedef MyU16 MyU16Array[10];


    template <typename Trait>
    class WindowControlInterface : public Trait::Base
    {
        public:
        using Trait::Base::Base;

        typename Trait::template Event<WindowControl> window_control{*this, "window_control"};
        typename Trait::template Event<MyU16Array> window_counter{*this, "window_counter"};
    };

    template <typename Trait>
    class WindowInfoInterface : public Trait::Base
    {
        public:
        using Trait::Base::Base;

        typename Trait::template Event<WindowInfo> window_info{*this, "window_info"};
    };


    using WindowControlProxy = score::mw::com::AsProxy<WindowControlInterface>;
    using WindowControlSkeleton = score::mw::com::AsSkeleton<WindowControlInterface>;
    using WindowInfoProxy = score::mw::com::AsProxy<WindowInfoInterface>;
    using WindowInfoSkeleton = score::mw::com::AsSkeleton<WindowInfoInterface>;

}
#endif // SCORE_CAR_WINDOW_TYPES_H
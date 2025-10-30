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

use std::default::Default;
/*
 * Enumerator that specifies the possible commands to control the car window
 */
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCommand {
  Stop = 0,
  Open = 1,
  Close = 2,
  Exit = 3,
}

/* Default implementation for WindowCommand */
impl Default for WindowCommand {
    fn default() -> Self {
        WindowCommand::Stop
    }
}

/*
 * Enumerator that specifies the possible states of the car window
 */
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
  Stopped = 0,
  Opening = 1,
  Closing = 2,
  Open = 3,
  Closed = 4,
  Shutdown = 5,
}

/* Default implementation for WindowState */
impl Default for WindowState {
    fn default() -> Self {
        WindowState::Stopped
    }
}

/*
 * Control command structure for the car window
 */
#[repr(C)]
#[derive(Debug,Clone,Default)]
pub struct WindowControl {
    pub command: WindowCommand,
}

/*
 * Status and information structure of the car window
 */
#[repr(C)]
#[derive(Debug,Clone,Default)]
pub struct WindowInfo {
    pub state: WindowState,
    pub pos: u32,
    pub lastpos: [u32;5],
}

/*
 * Type definition of uint16 as MyU16
 */
#[repr(C)]
#[derive(Debug,Clone,Default)]
pub struct MyU16(pub u16);

/*
 * Type definition of an array of 10 uint16
 */
#[repr(C)]
#[derive(Debug,Clone,Default)]
pub struct MyU16Array(pub [MyU16;10]);

mw_com::import_type!(car_window_types_WindowControl_type, crate::WindowControl);
mw_com::import_type!(car_window_types_MyU16Array_type, crate::MyU16Array);
mw_com::import_type!(car_window_types_WindowInfo_type, crate::WindowInfo);

mw_com::import_interface!(car_window_types_WindowControl_interface, WindowControl, {
    window_control: Event<crate::WindowControl>, 
    window_counter: Event<crate::MyU16Array> 
});

mw_com::import_interface!(car_window_types_WindowInfo_interface, WindowInfo, {
    window_info: Event<crate::WindowInfo> 
});

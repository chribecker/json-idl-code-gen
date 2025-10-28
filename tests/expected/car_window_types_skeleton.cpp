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
#include "car_window_types.h"
#include "score/mw/com/impl/rust/bridge_macros.h"

BEGIN_EXPORT_MW_COM_INTERFACE(car_window_types_CarWindowControl_interface, ::car_window_types::CarWindowControlProxy, ::car_window_types::CarWindowControlSkeleton)
EXPORT_MW_COM_EVENT(car_window_types_CarWindowControl_interface, ::car_window_types::WindowControl, window_control)
END_EXPORT_MW_COM_INTERFACE()

BEGIN_EXPORT_MW_COM_INTERFACE(car_window_types_CarWindowInfo_interface, ::car_window_types::CarWindowInfoProxy, ::car_window_types::CarWindowInfoSkeleton)
EXPORT_MW_COM_EVENT(car_window_types_CarWindowInfo_interface, ::car_window_types::WindowInfo, window_info)
END_EXPORT_MW_COM_INTERFACE()


EXPORT_MW_COM_TYPE(car_window_types_WindowControl_type, ::car_window_types::WindowControl)
EXPORT_MW_COM_TYPE(car_window_types_WindowInfo_type, ::car_window_types::WindowInfo)
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
{% set rust_type_map = {
  'uint8':'u8',
  'uint16':'u16',
  'uint32':'u32',
  'uint64':'u64',
  'int8':'i8',
  'int16':'i16',
  'int32':'i32',
  'int64':'i64',
  'boolean':'bool',
  'char':'char',
  'float':'f32',
  'double':'f64',
  'string':'string',
} %}
{% for ns in item.namespaces: %}
{% for en in ns.enumerations: %}
/*
 * {{ en.description }}
 */
#[repr({{rust_type_map.get(en.datatype, en.datatype)}})]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum {{ en.name }} {
{% for lit in en.options %}
    {{lit.name }} = {{lit.value}},
{% endfor %}
}

/* Default implementation for {{ en.name }} */
impl Default for {{ en.name }} {
    fn default() -> Self {
        {{ en.name }}::{{ en.options[0].name }}
    }
}

{% endfor %}
{% for str in ns.structs: %}
/*
 * {{ str.description }}
 */
#[repr(C)]
#[derive(Debug,Clone,Default)]
pub struct {{ str.name }} {
{% for member in str.members %}
    pub {{member.name }}: {{rust_type_map.get(member.datatype, member.datatype)}},
{% endfor %}
}

mw_com::import_type!(my_{{str.name}}, crate::{{str.name}});

{% endfor %}

{% for ns2 in ns.namespaces: %}

{% if ns2.interface != None %}
mw_com::import_interface!(my_{{ns2.name}}, {{ns2.name}}, {
{% for ev in ns2.interface.events: %}
{% for input in ev.input: %}
    {{ input.name }}: Event<crate::{{ rust_type_map.get(input.datatype, input.datatype) }}>,
{% endfor %}
{% endfor %}
});

{% endif %}
{% endfor %}
{% endfor %}

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
#ifndef SCORE_IPC_BRIDGE_DATATYPE_H
#define SCORE_IPC_BRIDGE_DATATYPE_H

#include "score/mw/com/types.h"
{% set cpp_type_map = {
  'uint8':'std::uint8_t',
  'uint16':'std::uint16_t',
  'uint32':'std::uint32_t',
  'uint64':'std::uint64_t',
  'int8':'std::int8_t',
  'int16':'std::int16_t',
  'int32':'std::int32_t',
  'int64':'std::int64_t',
  'boolean':'bool',
  'char':'char',
  'float':'float',
  'double':'double',
  'string':'std::string',
} %}
{% for ns in item.namespaces: %}

namespace {{ns.name}} {
{% for en in ns.enumerations: %}
{% if en.description != None %}
    /*
     * {{ en.description }}
     */
{% endif %}
    enum class {{ en.name }} : {{cpp_type_map.get(en.datatype, en.datatype)}} {
{% for lit in en.options %}
    {% if lit.description != None -%}
        // {{ lit.description }}
    {% endif %}
        {{lit.name }} = {{lit.value}}{% if en.datatype.startswith('uint') %}U{% endif %},
{% endfor %}
    };

{% endfor %}
{% for str in ns.structs: %}
    {% if str.description != None %}
    /*
     * {{ str.description }}
     */
    {% endif %}
    struct {{ str.name }} {
        // {{ str.name }}() : {% for member in str.members %}{% if not loop.first %}, {% endif %}{{member.name}}(0){% endfor %}{}
        {{ str.name }}({{ str.name }}&&) = default;
        {{ str.name }}(const {{ str.name }}&) = default;
        {{ str.name }}& operator=({{ str.name }}&&) = default;
        {{ str.name }}& operator=(const {{ str.name }}&) = default;
{% for member in str.members %}
    {% if member.description != None %}
        // {{ member.description }}
    {% endif %}
        {{cpp_type_map.get(member.datatype, member.datatype)}} {{member.name }};
{% endfor %}
    };

{% endfor %}
{% for ns2 in ns.namespaces: %}
{% if ns2.interface != None %}
    template <typename Trait>
    class {{ns2.name}}Interface : public Trait::Base
    {
        public:
        using Trait::Base::Base;

{% for ev in ns2.interface.events: %}
{% for input in ev.input: %}
        typename Trait::template Event<{{ cpp_type_map.get(input.datatype, input.datatype) }}> {{ input.name }}{*this, "{{ ev.name }}"};
{% endfor %}
{% endfor %}
    };

{% endif %}
{% endfor %}
{% endfor %}
}

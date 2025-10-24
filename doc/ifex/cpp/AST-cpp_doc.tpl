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
#include "src/{{ns.name}}.h"
{% endfor %}
#include "score/mw/com/impl/rust/bridge_macros.h"

{% for ns in item.namespaces: %}
{% for ns2 in ns.namespaces: %}
{% if ns2.interface != None %}
BEGIN_EXPORT_MW_COM_INTERFACE(my_{{ns2.name}}, ::{{ns.name}}::{{ns2.name}}Proxy, ::{{ns.name}}::{{ns2.name}}Skeleton)
{% for ev in ns2.interface.events: %}
{% for input in ev.input: %}
EXPORT_MW_COM_EVENT(my_{{ns2.name}}, ::{{ns.name}}::{{ cpp_type_map.get(input.datatype, input.datatype)}}, {{ input.name }})
{% endfor %}
{% endfor %}
END_EXPORT_MW_COM_INTERFACE()

{% endif %}
{% endfor %}
{% for en in ns.enumerations: %}
EXPORT_MW_COM_TYPE(my_{{ en.name }}, ::{{ ns.name }}::{{ en.name }})
{% endfor %}
{% for str in ns.structs: %}
EXPORT_MW_COM_TYPE(my_{{ str.name }}, ::{{ ns.name }}::{{ str.name }})
{% endfor %}

{% endfor %}

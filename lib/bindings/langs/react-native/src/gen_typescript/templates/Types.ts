{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Record { name, module_path } %}
{% include "RecordTemplate.ts" %}
{%- when Type::Enum { name, module_path } %}
{%- include "EnumTemplate.ts" %}
{%- else %}
{%- endmatch -%}    
{%- endfor %}

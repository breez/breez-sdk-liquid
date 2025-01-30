{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Object { name, module_path, imp } %}
{% let obj = ci.get_object_definition(name).unwrap() %}
{%- call ts::docstring(obj, 0, ci) %}
{%- for func in obj.methods() -%}
{%- if func.name()|ignored_function == false -%}
{%- include "TopLevelFunctionTemplate.ts" %}
{% endif -%}
{% endfor %}
{%- else -%}
{%- endmatch -%}    
{%- endfor %}


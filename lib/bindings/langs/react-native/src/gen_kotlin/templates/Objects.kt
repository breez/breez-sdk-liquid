{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name(ci) %}
{%- match type_ %}
{%- when Type::Object { module_path, name, imp } %}
{% let obj = ci.get_object_definition(name).unwrap() %}
{% let obj_interface = "getBindingLiquidSdk()." %}
{%- for func in obj.methods() -%}
{%- if func.name()|ignored_function == false -%}
{%- include "TopLevelFunctionTemplate.kt" %}
{% endif -%}
{% endfor %}
{%- else -%}
{%- endmatch -%}    
{%- endfor %}


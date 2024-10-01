{%- if !ci.is_name_used_as_error(name) -%}
{%- let e = ci.get_enum_definition(name).unwrap() %}
{%- if e.is_flat() %}
{% call ts::docstring(e, 0, ci) %}
export enum {{ type_name }} {
    {%- for variant in e.variants() -%}
    {%- call ts::docstring(variant, 1, ci) %}
    {{ variant.name()|enum_variant }} = "{{ variant.name()|var_name }}"{% if !loop.last %},{% endif %}
    {%- endfor %}
}

{%- else %}

export enum {{ type_name }}Variant {
    {%- for variant in e.variants() -%}
    {%- call ts::docstring(variant, 1, ci) %}
    {{ variant.name()|enum_variant }} = "{{ variant.name()|var_name }}"{% if !loop.last %},{% endif %}
    {%- endfor %}
}
{% call ts::docstring(e, 0, ci) %}
export type {{ type_name }} = {% for variant in e.variants() -%}{
    type: {{ type_name }}Variant.{{ variant.name()|enum_variant }}{% if variant.has_fields() %}, 
    {%- call ts::field_list_decl(variant) -%}{% endif %}
}{% if !loop.last %} | {% endif %}
{%- endfor %}

{%- endif %}
{%- endif %}
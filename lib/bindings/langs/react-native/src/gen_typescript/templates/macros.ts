
{% macro arg_list(func) %}
    {%- for arg in func.arguments() -%}
        {{ arg.name()|var_name -}}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{%- macro field_list(rec) %}
    {%- for f in rec.fields() %}
        {{ f.name()|var_name|unquote }},
    {%- endfor %}
{%- endmacro -%}

{%- macro field_list_decl(rec) %}
    {%- for f in rec.fields() %}
    {%- call docstring(f, 1, ci) %}
    {%- match f.as_type() %}
    {%- when Type::Optional { inner_type } %}
    {%- let unboxed = inner_type.as_ref() %}
    {{ f.name()|var_name }}?: {{ unboxed|type_name }}
    {%- else %}
    {{ f.name()|var_name }}: {{ f|type_name }}
    {%- endmatch %}
    {%- endfor %}
{%- endmacro -%}

{% macro arg_list_decl(func) %}
    {%- for arg in func.arguments() -%}
        {{ arg.name()|var_name }}: {{ arg|absolute_type_name }}{{- arg|default_value -}}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{%- macro docstring(defn, indent_tabs, ci) %}
{%- match defn.docstring() %}
{%- when Some(docstring) %}
{{ docstring|docstring(indent_tabs, ci) }}
{%- else -%}
{%- endmatch %}
{%- endmacro %}
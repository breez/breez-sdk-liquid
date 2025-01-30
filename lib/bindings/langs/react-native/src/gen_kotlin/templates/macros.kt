
{% macro arg_list(func) %}
    {%- for arg in func.arguments() -%}
        {%- match arg.as_type() -%}         
        {%- when Type::Enum { name, module_path } -%}
        {{ arg.name()|var_name|unquote|temporary }}
        {%- when Type::Optional { inner_type } -%}
        {{ arg.name()|var_name|unquote|temporary }}
        {%- when Type::Record { name, module_path } -%}
        {{ arg|type_name(ci)|var_name|unquote -}}
        {%- else -%}
        {{ arg.name()|var_name|unquote }}{{ arg|rn_convert_type(ci) -}}
        {%- endmatch -%}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{% macro arg_list_decl(func) %}
    {%- for arg in func.arguments() -%}
    {{- arg.name()|var_name|unquote }}: {{ arg|rn_type_name(ci) -}}, {% endfor %}
{%- endmacro %}

{%- macro field_list(rec) %}
    {%- for f in rec.fields() -%}
        {{ f.name()|var_name|unquote }},
    {%- endfor %}
{%- endmacro -%}

{% macro return_value(ret_type) %}   
    {%- match ret_type %}
    {%- when Type::Enum { name, module_path } %}readableMapOf(res)
    {%- when Type::Record { name, module_path } %}readableMapOf(res)
    {%- when Type::Sequence { inner_type } %}readableArrayOf(res)
    {%- else %}res
    {%- endmatch %}
{%- endmacro %}
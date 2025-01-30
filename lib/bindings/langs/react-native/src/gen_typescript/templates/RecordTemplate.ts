{%- let rec = ci.get_record_definition(name).unwrap() %}

{%- call ts::docstring(rec, 0, ci) %}
export type {{ type_name }} = {
    {%- call ts::field_list_decl(rec) %}
}

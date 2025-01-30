{%- let rec = ci.get_record_definition(name).unwrap() %}
fun as{{ type_name }}({{ type_name|var_name|unquote }}: ReadableMap): {{ type_name }}? {
    if (!validateMandatoryFields({{ type_name|var_name|unquote }}, arrayOf(
        {%- for field in rec.fields() %}
            {%- match field.as_type() %} 
            {%- when Type::Optional { inner_type } %}
            {%- else %}
            "{{ field.name()|var_name |unquote }}",
            {%- endmatch %}
        {%- endfor %}
    ))) {
        return null
    }

    {%- for field in rec.fields() %}
    val {{field.name()|var_name|unquote}} = {{ field|render_from_map(ci, type_name|var_name|unquote, field.name()|var_name|unquote, false) }}    
    {%- endfor %}
    return {{ type_name }}({%- call kt::field_list(rec) -%})    
}

fun readableMapOf({{ type_name|var_name|unquote }}: {{ type_name }}): ReadableMap {
    return readableMapOf(
        {%- for field in rec.fields() %}
            {%- match field.as_type() %} 
            {%- when Type::Optional { inner_type } %}
                {%- let unboxed = inner_type.as_ref() %}
                {%- match unboxed %}
                {%- when Type::Sequence { inner_type } %}
                {{- self.add_sequence_type(inner_type|type_name(ci)) }}
                {%- else %}
                {%- endmatch %}
            {%- when Type::Sequence { inner_type } %}
            {{- self.add_sequence_type(inner_type|type_name(ci)) }}
            {%- else %}
            {%- endmatch %}
            "{{ field.name()|var_name|unquote }}" to {{ field|render_to_map(ci, type_name|var_name|unquote, field.name()|var_name|unquote, false) }},
        {%- endfor %}       
    )
}

fun as{{ type_name }}List(arr: ReadableArray): List<{{ type_name }}> {
    val list = ArrayList<{{ type_name }}>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(as{{ type_name }}(value)!!)            
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}
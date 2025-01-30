    
    @ReactMethod
    fun {{ func.name()|fn_name|unquote }}({%- call kt::arg_list_decl(func) -%}promise: Promise) {
        executor.execute {
            try {
{%- for arg in func.arguments() -%}
    {%- match arg.as_type() %}
    {%- when Type::Enum { name, module_path } %}
        {%- let e = ci.get_enum_definition(name).unwrap() %}
        {%- if e.is_flat() %}
                val {{arg.name()|var_name|unquote|temporary}} = as{{arg|type_name(ci)}}({{ arg.name()|var_name|unquote }})
        {%- else %}
                val {{arg.name()|var_name|unquote|temporary}} = as{{arg|type_name(ci)}}({{ arg.name()|var_name|unquote }}) ?: run { throw SdkException.Generic(errMissingMandatoryField("{{arg.name()|var_name|unquote}}", "{{ arg|type_name(ci) }}")) }
        {%- endif %}
    {%- when Type::Optional { inner_type } %}
                val {{arg.name()|var_name|unquote|temporary}} = {{arg.name()|var_name|unquote}}{{ arg|rn_convert_type(ci) -}}
    {%- when Type::Record { name, module_path } %}
                val {{arg|type_name(ci)|var_name|unquote}} = as{{arg|type_name(ci)}}({{ arg.name()|var_name|unquote }}) ?: run { throw SdkException.Generic(errMissingMandatoryField("{{arg.name()|var_name|unquote}}", "{{ arg|type_name(ci) }}")) }
    {%- else %}
    {%- endmatch %}
{%- endfor %}
{%- match func.return_type() -%}
{%- when Some with (return_type) %}
                val res = {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call kt::arg_list(func) -%})
{%- if func.name() == "default_config" %}
                val workingDir = File(reactApplicationContext.filesDir.toString() + "/breezSdkLiquid")

                res.workingDir = workingDir.absolutePath
{%- endif -%}               
    {%- match return_type %}
    {%- when Type::Optional { inner_type } %}
        {%- let unboxed = inner_type.as_ref() %}
                promise.resolve(res?.let { {% call kt::return_value(unboxed) %} })
    {%- else %}
                promise.resolve({% call kt::return_value(return_type) %})
    {%- endmatch %}
{%- when None %}
                {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call kt::arg_list(func) -%})
{%- if func.name() == "disconnect" %}
                bindingLiquidSdk = null
{%- endif %}
                promise.resolve(readableMapOf("status" to "ok"))
{%- endmatch %}
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }
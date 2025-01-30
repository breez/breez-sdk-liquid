    
    @objc({%- call swift::extern_arg_list(func) -%})
    func {{ func.name()|fn_name|unquote }}(_ {% call swift::arg_list_decl(func) -%}resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
{%- for arg in func.arguments() -%}
    {%- match arg.as_type() %}
    {%- when Type::Enum { name, module_path } %}
        {%- let e = ci.get_enum_definition(name).unwrap() %}
        {%- if e.is_flat() %}
            let {{arg.name()|var_name|unquote|temporary}} = try BreezSDKLiquidMapper.as{{arg|type_name}}({{ arg|type_name|var_name|unquote }}: {{ arg.name()|var_name|unquote }})
        {%- else %}
            let {{arg.name()|var_name|unquote|temporary}} = try BreezSDKLiquidMapper.as{{arg|type_name}}({{ arg|type_name|var_name|unquote }}: {{ arg.name()|var_name|unquote }})
        {%- endif %}
    {%- when Type::Optional { inner_type } %}
            let {{arg.name()|var_name|unquote|temporary}} = {{ arg|rn_convert_type(arg.name()|var_name|unquote) -}}
    {%- when Type::Record { name, module_path } %}
            let {{arg|type_name|var_name|unquote}} = try BreezSDKLiquidMapper.as{{arg|type_name}}({{ arg|type_name|var_name|unquote }}: {{ arg.name()|var_name|unquote }})
    {%- else %}
    {%- endmatch %}
{%- endfor %}
{%- match func.return_type() -%}
{%- when Some with (return_type) %}
            var res = {%- call swift::throws_decl(func) -%}{{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call swift::arg_list(func) -%})
{%- if func.name() == "default_config" %}
            res.workingDir = RNBreezSDKLiquid.breezSdkLiquidDirectory.path
{%- endif -%}
    {%- match return_type %}
    {%- when Type::Optional { inner_type } %}
        {%- let unboxed = inner_type.as_ref() %}
            if res != nil {
                resolve({{ unboxed|rn_return_type(unboxed|type_name|var_name|unquote, true) }})
            } else {
                resolve(nil)
            }
    {%- else %}
            resolve({{ return_type|rn_return_type(return_type|type_name|var_name|unquote, false) }})
    {%- endmatch %}
{%- when None %}
            try {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call swift::arg_list(func) -%})
{%- if func.name() == "disconnect" %}
            bindingLiquidSdk = nil
{%- endif %}
            resolve(["status": "ok"])     
{%- endmatch %}
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
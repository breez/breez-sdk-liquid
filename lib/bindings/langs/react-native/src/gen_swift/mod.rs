use std::collections::HashSet;
use std::fmt::Debug;

use askama::Template;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use once_cell::sync::Lazy;
use uniffi_bindgen::interface::*;

use crate::generator::Config;

mod callback_interface;
mod compounds;
mod custom;
mod enum_;
mod executor;
mod external;
mod miscellany;
mod object;
mod primitives;
mod record;

/// A trait tor the implementation.
#[allow(dead_code)]
trait CodeType: Debug {
    /// The language specific label used to reference this type. This will be used in
    /// method signatures and property declarations.
    fn type_label(&self) -> String;

    /// A representation of this type label that can be used as part of another
    /// identifier. e.g. `read_foo()`, or `FooInternals`.
    ///
    /// This is especially useful when creating specialized objects or methods to deal
    /// with this type only.
    fn canonical_name(&self) -> String {
        self.type_label()
    }

    fn literal(&self, _literal: &Literal) -> String {
        unimplemented!("Unimplemented for {}", self.type_label())
    }

    /// Name of the FfiConverter
    ///
    /// This is the object that contains the lower, write, lift, and read methods for this type.
    fn ffi_converter_name(&self) -> String {
        format!("FfiConverter{}", self.canonical_name())
    }

    // XXX - the below should be removed and replace with the ffi_converter_name reference in the template.
    /// An expression for lowering a value into something we can pass over the FFI.
    fn lower(&self) -> String {
        format!("{}.lower", self.ffi_converter_name())
    }

    /// An expression for writing a value into a byte buffer.
    fn write(&self) -> String {
        format!("{}.write", self.ffi_converter_name())
    }

    /// An expression for lifting a value from something we received over the FFI.
    fn lift(&self) -> String {
        format!("{}.lift", self.ffi_converter_name())
    }

    /// An expression for reading a value from a byte buffer.
    fn read(&self) -> String {
        format!("{}.read", self.ffi_converter_name())
    }

    /// A list of imports that are needed if this type is in use.
    /// Classes are imported exactly once.
    fn imports(&self) -> Option<Vec<String>> {
        None
    }

    /// Function to run at startup
    fn initialization_fn(&self) -> Option<String> {
        None
    }
}

static IGNORED_FUNCTIONS: Lazy<HashSet<String>> = Lazy::new(|| {
    let list: Vec<&str> = vec![
        "connect",
        "add_event_listener",
        "set_logger",
        "connect_with_signer",
    ];
    HashSet::from_iter(list.into_iter().map(|s| s.to_string()))
});

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "mapper.swift")]
#[allow(dead_code)]
pub struct MapperGenerator<'a> {
    config: Config,
    ci: &'a ComponentInterface,
}

impl<'a> MapperGenerator<'a> {
    pub fn new(config: Config, ci: &'a ComponentInterface) -> Self {
        Self { config, ci }
    }
}

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "extern.m")]
#[allow(dead_code)]
pub struct ExternGenerator<'a> {
    config: Config,
    ci: &'a ComponentInterface,
}

impl<'a> ExternGenerator<'a> {
    pub fn new(config: Config, ci: &'a ComponentInterface) -> Self {
        Self { config, ci }
    }
}

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "module.swift")]
#[allow(dead_code)]
pub struct ModuleGenerator<'a> {
    config: Config,
    ci: &'a ComponentInterface,
}

impl<'a> ModuleGenerator<'a> {
    pub fn new(config: Config, ci: &'a ComponentInterface) -> Self {
        Self { config, ci }
    }
}

#[derive(Clone)]
pub struct SwiftCodeOracle;

#[allow(dead_code)]
impl SwiftCodeOracle {
    // Map `Type` instances to a `Box<dyn CodeType>` for that type.
    //
    // There is a companion match in `templates/Types.swift` which performs a similar function for the
    // template code.
    //
    //   - When adding additional types here, make sure to also add a match arm to the `Types.swift` template.
    //   - To keep things manageable, let's try to limit ourselves to these 2 mega-matches
    fn create_code_type(&self, type_: Type) -> Box<dyn CodeType> {
        match type_ {
            Type::UInt8 => Box::new(primitives::UInt8CodeType),
            Type::Int8 => Box::new(primitives::Int8CodeType),
            Type::UInt16 => Box::new(primitives::UInt16CodeType),
            Type::Int16 => Box::new(primitives::Int16CodeType),
            Type::UInt32 => Box::new(primitives::UInt32CodeType),
            Type::Int32 => Box::new(primitives::Int32CodeType),
            Type::UInt64 => Box::new(primitives::UInt64CodeType),
            Type::Int64 => Box::new(primitives::Int64CodeType),
            Type::Float32 => Box::new(primitives::Float32CodeType),
            Type::Float64 => Box::new(primitives::Float64CodeType),
            Type::Boolean => Box::new(primitives::BooleanCodeType),
            Type::String => Box::new(primitives::StringCodeType),
            Type::Bytes => Box::new(primitives::BytesCodeType),

            Type::Timestamp => Box::new(miscellany::TimestampCodeType),
            Type::Duration => Box::new(miscellany::DurationCodeType),

            Type::Enum { name, .. } => Box::new(enum_::EnumCodeType::new(name)),
            Type::Object { name, .. } => Box::new(object::ObjectCodeType::new(name)),
            Type::Record { name, .. } => Box::new(record::RecordCodeType::new(name)),
            Type::CallbackInterface { name, .. } => {
                Box::new(callback_interface::CallbackInterfaceCodeType::new(name))
            }
            Type::ForeignExecutor => Box::new(executor::ForeignExecutorCodeType),
            Type::Optional { inner_type } => {
                Box::new(compounds::OptionalCodeType::new(*inner_type))
            }
            Type::Sequence { inner_type } => {
                Box::new(compounds::SequenceCodeType::new(*inner_type))
            }
            Type::Map {
                key_type,
                value_type,
            } => Box::new(compounds::MapCodeType::new(*key_type, *value_type)),
            Type::External { name, .. } => Box::new(external::ExternalCodeType::new(name)),
            Type::Custom { name, .. } => Box::new(custom::CustomCodeType::new(name)),
        }
    }

    fn find(&self, type_: &impl AsType) -> Box<dyn CodeType> {
        self.create_code_type(type_.as_type())
    }

    /// Get the idiomatic Swift rendering of a class name (for enums, records, errors, etc).
    fn class_name(&self, nm: &str) -> String {
        nm.to_string().to_upper_camel_case()
    }

    /// Get the idiomatic Swift rendering of a function name.
    fn fn_name(&self, nm: &str) -> String {
        nm.to_string().to_lower_camel_case()
    }

    /// Get the idiomatic Swift rendering of a variable name.
    fn var_name(&self, nm: &str) -> String {
        nm.to_string().to_lower_camel_case()
    }

    /// Get the idiomatic Swift rendering of an individual enum variant.
    fn enum_variant_name(&self, nm: &str) -> String {
        nm.to_string().to_lower_camel_case()
    }
}

pub mod filters {
    use super::*;

    fn oracle() -> &'static SwiftCodeOracle {
        &SwiftCodeOracle
    }

    pub fn type_name(as_type: &impl AsType) -> Result<String, askama::Error> {
        Ok(oracle().find(as_type).type_label())
    }

    pub fn fn_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().fn_name(nm))
    }

    pub fn render_to_map(
        type_: &impl AsType,
        ci: &ComponentInterface,
        obj_name: &str,
        field_name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let type_name = filters::type_name(type_)?;
        let type_name_str = type_name.as_str();
        let var_name = filters::unquote(filters::var_name(type_name_str)?.as_str())?;
        let mut obj_prefix = "".to_string();
        if !obj_name.is_empty() {
            obj_prefix = format!("{obj_name}.");
        }
        let mut optional_suffix = "";
        if optional {
            optional_suffix = "!";
        }
        let res: Result<String, askama::Error> = match type_.as_type() {
            Type::UInt8 => Ok(format!("{obj_prefix}{field_name}")),
            Type::Int8 => Ok(format!("{obj_prefix}{field_name}")),
            Type::UInt16 => Ok(format!("{obj_prefix}{field_name}")),
            Type::Int16 => Ok(format!("{obj_prefix}{field_name}")),
            Type::UInt32 => Ok(format!("{obj_prefix}{field_name}")),
            Type::Int32 => Ok(format!("{obj_prefix}{field_name}")),
            Type::UInt64 => Ok(format!("{obj_prefix}{field_name}")),
            Type::Int64 => Ok(format!("{obj_prefix}{field_name}")),
            Type::Float32 => Ok(format!("{obj_prefix}{field_name}")),
            Type::Float64 => Ok(format!("{obj_prefix}{field_name}")),
            Type::Boolean => Ok(format!("{obj_prefix}{field_name}")),
            Type::String => Ok(format!("{obj_prefix}{field_name}")),
            Type::Bytes => Ok(format!("{obj_prefix}{field_name}")),
            Type::Timestamp => unimplemented!("render_to_map: Timestamp is not implemented"),
            Type::Duration => unimplemented!("render_to_map: Duration is not implemented"),
            Type::Object { .. } => unimplemented!("render_to_map: Object is not implemented"),
            Type::Record { .. } => Ok(format!(
                "dictionaryOf({var_name}: {obj_prefix}{field_name}{optional_suffix})"
            )),
            Type::Enum { name, .. } => {
                let enum_def = ci.get_enum_definition(&name).unwrap();
                match enum_def.is_flat() {
                    true => Ok(format!(
                        "valueOf( {var_name}: {obj_prefix}{field_name}{optional_suffix})"
                    )),
                    false => Ok(format!(
                        "dictionaryOf({var_name}: {obj_prefix}{field_name}{optional_suffix})"
                    )),
                }
            }
            Type::CallbackInterface { .. } => {
                unimplemented!("render_to_map: CallbackInterface is not implemented")
            }
            Type::ForeignExecutor { .. } => {
                unimplemented!("render_to_map: ForeignExecutor is not implemented")
            }
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                let inner_render = render_to_map(unboxed, ci, obj_name, field_name, true)?;
                Ok(format!(
                    "{obj_prefix}{field_name} == nil ? nil : {inner_render}"
                ))
            }
            Type::Sequence { inner_type } => {
                let unboxed = inner_type.as_ref();
                let type_name = filters::type_name(unboxed)?;
                let var_name = filters::var_name(type_name.as_str())?;
                let var_name = filters::unquote(var_name.as_str())?;
                let as_array_statment = match unboxed {
                    Type::Record { .. } => format!(
                        "arrayOf({var_name}List: {obj_prefix}{field_name}{optional_suffix})"
                    ),
                    Type::Enum { .. } => format!(
                        "arrayOf({var_name}List: {obj_prefix}{field_name}{optional_suffix})"
                    ),
                    _ => format!("{obj_prefix}{field_name}"),
                };
                Ok(as_array_statment)
            }
            Type::Map { .. } => unimplemented!("render_to_map: Map is not implemented"),
            Type::External { .. } => {
                unimplemented!("render_to_map: External is not implemented")
            }
            Type::Custom { .. } => {
                unimplemented!("render_to_map: Custom is not implemented")
            }
        };
        res
    }

    pub fn rn_convert_type(
        type_: &impl AsType,
        converted_var_name: &str,
    ) -> Result<String, askama::Error> {
        match type_.as_type() {
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                let optional = match *unboxed {
                    Type::Int8
                    | Type::UInt8
                    | Type::Int16
                    | Type::UInt16
                    | Type::Int32
                    | Type::UInt32
                    | Type::Int64
                    | Type::UInt64 => {
                        format!("{} == 0 ? nil : {}", converted_var_name, converted_var_name)
                    }
                    Type::Float32 | Type::Float64 => format!(
                        "{} == 0.0 ? nil : {}",
                        converted_var_name, converted_var_name
                    ),
                    Type::String => format!(
                        "{}.isEmpty ? nil : {}",
                        converted_var_name, converted_var_name
                    ),
                    Type::Bytes => format!(
                        "{}.isEmpty ? nil : {}",
                        converted_var_name, converted_var_name
                    ),
                    _ => "".to_string(),
                };
                Ok(optional.to_string())
            }
            _ => Ok(converted_var_name.to_string()),
        }
    }

    pub fn rn_return_type(
        type_: &impl AsType,
        name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let mut optional_suffix = "";
        if optional {
            optional_suffix = "!";
        }
        match type_.as_type() {
            Type::Enum { .. } | Type::Record { .. } => Ok(format!(
                "BreezSDKLiquidMapper.dictionaryOf({}: res{})",
                name, optional_suffix
            )),
            Type::Sequence { inner_type } => {
                let unboxed = inner_type.as_ref();
                match unboxed {
                    Type::Enum { .. } | Type::Record { .. } => Ok(format!(
                        "BreezSDKLiquidMapper.arrayOf({}List: res{})",
                        name, optional_suffix
                    )),
                    _ => Ok(format!("res{}", optional_suffix)),
                }
            }
            _ => Ok(format!("res{}", optional_suffix)),
        }
    }

    pub fn rn_type_name(
        type_: &impl AsType,
        ci: &ComponentInterface,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let mut optional_suffix = "";
        if optional {
            optional_suffix = "?";
        }
        match type_.as_type() {
            Type::Record { .. } => Ok(format!("[String: Any{}]", optional_suffix)),
            Type::Enum { name, .. } => {
                let enum_def = ci.get_enum_definition(&name).unwrap();
                match enum_def.is_flat() {
                    false => Ok(format!("[String: Any{}]", optional_suffix)),
                    true => Ok("String".into()),
                }
            }
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                rn_type_name(unboxed, ci, optional)
            }
            Type::Sequence { inner_type } => {
                let unboxed = inner_type.as_ref();
                Ok(format!("[{}]", rn_type_name(unboxed, ci, optional)?))
            }
            _ => {
                let name = filters::type_name(type_)?;
                Ok(name.to_string())
            }
        }
    }

    pub fn extern_type_name(
        type_: &impl AsType,
        ci: &ComponentInterface,
    ) -> Result<String, askama::Error> {
        match type_.as_type() {
            Type::Boolean => Ok("BOOL".to_string()),
            Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 => Ok("NSInteger*".to_string()),
            Type::UInt8 | Type::UInt16 | Type::UInt32 | Type::UInt64 => {
                Ok("NSUInteger*".to_string())
            }
            Type::Float32 | Type::Float64 => Ok("NSNumber*".to_string()),
            Type::String => Ok("NSString*".to_string()),
            Type::Enum { name, .. } => {
                let enum_def = ci.get_enum_definition(&name).unwrap();
                match enum_def.is_flat() {
                    false => Ok("NSDictionary*".to_string()),
                    true => Ok("NSString*".to_string()),
                }
            }
            Type::Record { .. } => Ok("NSDictionary*".to_string()),
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                extern_type_name(unboxed, ci)
            }
            Type::Bytes { .. } | Type::Sequence { .. } => Ok("NSArray*".to_string()),
            _ => Ok("".to_string()),
        }
    }

    pub fn inline_optional_field(
        type_: &impl AsType,
        ci: &ComponentInterface,
    ) -> Result<bool, askama::Error> {
        match type_.as_type() {
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                inline_optional_field(unboxed, ci)
            }
            _ => {
                let mapped_name = filters::rn_type_name(type_, ci, true)?;
                let type_name = filters::type_name(type_)?;
                Ok(mapped_name == type_name)
            }
        }
    }

    pub fn render_from_map(
        type_: &impl AsType,
        ci: &ComponentInterface,
        map_var_name: &str,
    ) -> Result<String, askama::Error> {
        let res: String = match type_.as_type() {
            Type::UInt8 => map_var_name.to_string(),
            Type::Int8 => map_var_name.to_string(),
            Type::UInt16 => map_var_name.to_string(),
            Type::Int16 => map_var_name.to_string(),
            Type::UInt32 => map_var_name.to_string(),
            Type::Int32 => map_var_name.to_string(),
            Type::UInt64 => map_var_name.to_string(),
            Type::Int64 => map_var_name.to_string(),
            Type::Float32 => map_var_name.to_string(),
            Type::Float64 => map_var_name.to_string(),
            Type::Boolean => map_var_name.to_string(),
            Type::String => map_var_name.to_string(),
            Type::Bytes => map_var_name.to_string(),
            Type::Timestamp => "".into(),
            Type::Duration => "".into(),
            Type::Object { .. } => "".into(),
            Type::Record { .. } => {
                let record_type_name = type_name(type_)?;
                let record_var_name = var_name(&record_type_name)?;
                let record_unquoted_name = unquote(&record_var_name)?;
                format!("try as{record_type_name}({record_unquoted_name}: {map_var_name})")
            }
            Type::Enum { name, .. } => {
                let enum_def = ci.get_enum_definition(&name).unwrap();
                let enum_var_name = var_name(&name)?;
                let enum_unquoted_name = unquote(&enum_var_name)?;
                match enum_def.is_flat() {
                    false => format!("try as{name}({enum_unquoted_name}: {map_var_name})"),
                    true => format!("try as{name}({enum_unquoted_name}: {map_var_name})"),
                }
            }
            Type::CallbackInterface { .. } => "".into(),
            Type::ForeignExecutor { .. } => "".into(),
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();

                render_from_map(unboxed, ci, map_var_name)?
            }
            Type::Sequence { inner_type } => {
                let unboxed = inner_type.as_ref();
                let element_type_name = type_name(unboxed)?;
                match unboxed {
                    Type::Enum { .. } | Type::Record { .. } => {
                        format!("try as{element_type_name}List(arr: {map_var_name})")
                    }
                    _ => map_var_name.to_string(),
                }
            }
            Type::Map { .. } => "".into(),
            Type::External { .. } => "".into(),
            Type::Custom { .. } => "".into(),
        };
        Ok(res.to_string())
    }

    pub fn var_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().var_name(nm))
    }

    pub fn unquote(nm: &str) -> Result<String, askama::Error> {
        Ok(nm.trim_matches('`').to_string())
    }

    pub fn ignored_function(nm: &str) -> Result<bool, askama::Error> {
        Ok(IGNORED_FUNCTIONS.contains(nm))
    }

    pub fn list_arg(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("`{nm}List`"))
    }

    pub fn temporary(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("{nm}Tmp"))
    }
}

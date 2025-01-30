use std::cell::RefCell;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;

use askama::Template;
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToUpperCamelCase};
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

#[allow(dead_code)]
trait CodeType: Debug {
    /// The language specific label used to reference this type. This will be used in
    /// method signatures and property declarations.
    fn type_label(&self, ci: &ComponentInterface) -> String;

    /// A representation of this type label that can be used as part of another
    /// identifier. e.g. `read_foo()`, or `FooInternals`.
    ///
    /// This is especially useful when creating specialized objects or methods to deal
    /// with this type only.
    fn canonical_name(&self) -> String;

    fn literal(&self, _literal: &Literal, ci: &ComponentInterface) -> String {
        unimplemented!("Unimplemented for {}", self.type_label(ci))
    }

    /// Name of the FfiConverter
    ///
    /// This is the object that contains the lower, write, lift, and read methods for this type.
    /// Depending on the binding this will either be a singleton or a class with static methods.
    ///
    /// This is the newer way of handling these methods and replaces the lower, write, lift, and
    /// read CodeType methods.  Currently only used by Kotlin, but the plan is to move other
    /// backends to using this.
    fn ffi_converter_name(&self) -> String {
        format!("FfiConverter{}", self.canonical_name())
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
#[template(syntax = "rn", escape = "none", path = "mapper.kt")]
#[allow(dead_code)]
pub struct MapperGenerator<'a> {
    config: Config,
    ci: &'a ComponentInterface,
    // Track types used in sequences with the `add_sequence_type()` macro
    sequence_types: RefCell<BTreeSet<String>>,
}

impl<'a> MapperGenerator<'a> {
    pub fn new(config: Config, ci: &'a ComponentInterface) -> Self {
        Self {
            config,
            ci,
            sequence_types: RefCell::new(BTreeSet::new()),
        }
    }

    // Helper to add a sequence type
    //
    // Call this inside your template to add a type used in a sequence.
    // This type is then added to the pushToArray helper.
    // Imports will be sorted and de-deuped.
    //
    // Returns an empty string so that it can be used inside an askama `{{ }}` block.
    fn add_sequence_type(&self, type_name: &str) -> &str {
        self.sequence_types
            .borrow_mut()
            .insert(type_name.to_owned());
        ""
    }

    pub fn sequence_types(&self) -> Vec<String> {
        let sequence_types = self.sequence_types.clone().into_inner();
        sequence_types.into_iter().collect()
    }
}

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "module.kt")]
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
pub struct KotlinCodeOracle;

#[allow(dead_code)]
impl KotlinCodeOracle {
    // Map `Type` instances to a `Box<dyn CodeType>` for that type.
    //
    // There is a companion match in `templates/Types.kt` which performs a similar function for the
    // template code.
    //
    //   - When adding additional types here, make sure to also add a match arm to the `Types.kt` template.
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

    /// Get the idiomatic Kotlin rendering of a class name (for enums, records, errors, etc).
    fn class_name(&self, ci: &ComponentInterface, nm: &str) -> String {
        let name = nm.to_string().to_upper_camel_case();
        // fixup errors.
        ci.is_name_used_as_error(nm)
            .then(|| self.convert_error_suffix(&name))
            .unwrap_or(name)
    }

    fn convert_error_suffix(&self, nm: &str) -> String {
        match nm.strip_suffix("Error") {
            None => nm.to_string(),
            Some(stripped) => format!("{stripped}Exception"),
        }
    }

    /// Get the idiomatic Kotlin rendering of a function name.
    fn fn_name(&self, nm: &str) -> String {
        format!("`{}`", nm.to_string().to_lower_camel_case())
    }

    /// Get the idiomatic Kotlin rendering of a variable name.
    fn var_name(&self, nm: &str) -> String {
        format!("`{}`", nm.to_string().to_lower_camel_case())
    }

    /// Get the idiomatic Kotlin rendering of an individual enum variant.
    fn enum_variant_name(&self, nm: &str) -> String {
        nm.to_string().to_shouty_snake_case()
    }
}

pub mod filters {
    use super::*;

    fn oracle() -> &'static KotlinCodeOracle {
        &KotlinCodeOracle
    }

    pub fn type_name(
        type_: &impl AsType,
        ci: &ComponentInterface,
    ) -> Result<String, askama::Error> {
        Ok(oracle().find(type_).type_label(ci))
    }

    pub fn fn_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().fn_name(nm))
    }

    pub fn render_to_array(
        type_name: &str,
        ci: &ComponentInterface,
    ) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match type_name {
            "Boolean" => Ok("array.pushBoolean(value)".to_string()),
            "Double" => Ok("array.pushDouble(value)".to_string()),
            "Int" => Ok("array.pushInt(value)".to_string()),
            "ReadableArray" => Ok("array.pushArray(value)".to_string()),
            "ReadableMap" => Ok("array.pushMap(value)".to_string()),
            "String" => Ok("array.pushString(value)".to_string()),
            "UByte" => Ok("array.pushInt(value.toInt())".to_string()),
            "UInt" => Ok("array.pushInt(value.toInt())".to_string()),
            "UShort" => Ok("array.pushInt(value.toInt())".to_string()),
            "ULong" => Ok("array.pushDouble(value.toDouble())".to_string()),
            _ => match ci.get_type(type_name) {
                Some(t) => match t {
                    Type::Enum { name, .. } => {
                        let enum_def = ci.get_enum_definition(&name).unwrap();
                        match enum_def.is_flat() {
                            true => Ok("array.pushString(value.name.lowercase())".to_string()),
                            false => Ok("array.pushMap(readableMapOf(value))".to_string()),
                        }
                    }
                    _ => Ok("array.pushMap(readableMapOf(value))".to_string()),
                },
                None => unimplemented!("known type: {type_name}"),
            },
        };
        res
    }

    pub fn render_to_map(
        type_: &impl AsType,
        ci: &ComponentInterface,
        obj_name: &str,
        field_name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match type_.as_type() {
            Type::UInt8 => Ok(format!("{obj_name}.{field_name}")),
            Type::Int8 => Ok(format!("{obj_name}.{field_name}")),
            Type::UInt16 => Ok(format!("{obj_name}.{field_name}")),
            Type::Int16 => Ok(format!("{obj_name}.{field_name}")),
            Type::UInt32 => Ok(format!("{obj_name}.{field_name}")),
            Type::Int32 => Ok(format!("{obj_name}.{field_name}")),
            Type::UInt64 => Ok(format!("{obj_name}.{field_name}")),
            Type::Int64 => Ok(format!("{obj_name}.{field_name}")),
            Type::Float32 => Ok(format!("{obj_name}.{field_name}")),
            Type::Float64 => Ok(format!("{obj_name}.{field_name}")),
            Type::Boolean => Ok(format!("{obj_name}.{field_name}")),
            Type::String => Ok(format!("{obj_name}.{field_name}")),
            Type::Bytes => Ok(format!("{obj_name}.{field_name}")),
            Type::Timestamp => unimplemented!("render_to_map: Timestamp is not implemented"),
            Type::Duration => unimplemented!("render_to_map: Duration is not implemented"),
            Type::Object { .. } => unimplemented!("render_to_map: Object is not implemented"),
            Type::Record { .. } => match optional {
                true => Ok(format!(
                    "{obj_name}.{field_name}?.let {{ readableMapOf(it) }}"
                )),
                false => Ok(format!("readableMapOf({obj_name}.{field_name})")),
            },
            Type::Enum { name, .. } => {
                let enum_def = ci.get_enum_definition(&name).unwrap();
                match enum_def.is_flat() {
                    true => match optional {
                        true => Ok(format!(
                            "{obj_name}.{field_name}?.let {{ it.name.lowercase() }}"
                        )),
                        false => Ok(format!("{obj_name}.{field_name}.name.lowercase()")),
                    },
                    false => match optional {
                        true => Ok(format!(
                            "{obj_name}.{field_name}?.let {{ readableMapOf(it) }}"
                        )),
                        false => Ok(format!("readableMapOf({obj_name}.{field_name})")),
                    },
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
                render_to_map(unboxed, ci, obj_name, field_name, true)
            }
            Type::Sequence { .. } => match optional {
                true => Ok(format!(
                    "{obj_name}.{field_name}?.let {{ readableArrayOf(it) }}"
                )),
                false => Ok(format!("readableArrayOf({obj_name}.{field_name})")),
            },
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

    pub fn render_from_map(
        type_: &impl AsType,
        ci: &ComponentInterface,
        name: &str,
        field_name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let mut mandatory_suffix = "";
        if !optional {
            mandatory_suffix = "!!"
        }
        let res: String = match type_.as_type() {
            Type::UInt8 => format!("{name}.getInt(\"{field_name}\").toUByte()"),
            Type::Int8 => format!("{name}.getInt(\"{field_name}\").toByte()"),
            Type::UInt16 => format!("{name}.getInt(\"{field_name}\").toUShort()"),
            Type::Int16 => format!("{name}.getInt(\"{field_name}\").toShort()"),
            Type::UInt32 => format!("{name}.getInt(\"{field_name}\").toUInt()"),
            Type::Int32 => format!("{name}.getInt(\"{field_name}\")"),
            Type::UInt64 => format!("{name}.getDouble(\"{field_name}\").toULong()"),
            Type::Int64 => format!("{name}.getDouble(\"{field_name}\").toLong()"),
            Type::Float32 => format!("{name}.getDouble(\"{field_name}\").toFloat()"),
            Type::Float64 => format!("{name}.getDouble(\"{field_name}\")"),
            Type::Boolean => format!("{name}.getBoolean(\"{field_name}\")"),
            Type::String => format!("{name}.getString(\"{field_name}\"){mandatory_suffix}"),
            Type::Bytes => format!("{name}.getString(\"{field_name}\").toByteArray()"),
            Type::Timestamp => "".into(),
            Type::Duration => "".into(),
            Type::Object { .. } => "".into(),
            Type::Record { .. } => {
                let record_type_name = type_name(type_, ci)?;
                format!(
                    "{name}.getMap(\"{field_name}\")?.let {{ as{record_type_name}(it)}}{mandatory_suffix}"
                )
            }
            Type::Enum {
                name: inner_name, ..
            } => {
                let enum_def = ci.get_enum_definition(&inner_name).unwrap();
                match enum_def.is_flat() {
                    false => {
                        format!("{name}.getMap(\"{field_name}\")?.let {{ as{inner_name}(it)}}{mandatory_suffix}")
                    }
                    true => format!(
                        "{name}.getString(\"{field_name}\")?.let {{ as{inner_name}(it)}}{mandatory_suffix}"
                    ),
                }
            }
            Type::CallbackInterface { .. } => "".into(),
            Type::ForeignExecutor { .. } => "".into(),
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                let inner_res = render_from_map(unboxed, ci, name, field_name, true)?;
                format!("if (hasNonNullKey({name}, \"{field_name}\")) {inner_res} else null")
            }
            Type::Sequence { inner_type } => {
                let unboxed = inner_type.as_ref();
                let element_type_name = type_name(unboxed, ci)?;
                format!("{name}.getArray(\"{field_name}\")?.let {{ as{element_type_name}List(it) }}{mandatory_suffix}")
            }
            Type::Map { .. } => "".into(),
            Type::External { .. } => "".into(),
            Type::Custom { .. } => "".into(),
        };
        Ok(res.to_string())
    }

    /// Get the idiomatic Kotlin rendering of a variable name.
    pub fn var_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().var_name(nm))
    }

    pub fn unquote(nm: &str) -> Result<String, askama::Error> {
        Ok(nm.trim_matches('`').to_string())
    }

    pub fn ignored_function(nm: &str) -> Result<bool, askama::Error> {
        Ok(IGNORED_FUNCTIONS.contains(nm))
    }

    pub fn rn_convert_type(
        type_: &impl AsType,
        _ci: &ComponentInterface,
    ) -> Result<String, askama::Error> {
        match type_.as_type() {
            Type::UInt8 | Type::UInt16 | Type::UInt32 => Ok(".toUInt()".to_string()),
            Type::Int64 => Ok(".toLong()".to_string()),
            Type::UInt64 => Ok(".toULong()".to_string()),
            Type::Float32 | Type::Float64 => Ok(".toFloat()".to_string()),
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                let conversion = rn_convert_type(unboxed, _ci).unwrap();
                let optional = match *unboxed {
                    Type::Int8
                    | Type::UInt8
                    | Type::Int16
                    | Type::UInt16
                    | Type::Int32
                    | Type::UInt32 => ".takeUnless { it == 0 }".to_string(),
                    Type::Int64 => ".takeUnless { it == 0L }".to_string(),
                    Type::UInt64 => ".takeUnless { it == 0UL }".to_string(),
                    Type::Float32 | Type::Float64 => ".takeUnless { it == 0.0 }".to_string(),
                    Type::String | Type::Bytes => ".takeUnless { it.isEmpty() }".to_string(),
                    _ => "".to_string(),
                };
                Ok(format!("{}{}", conversion, optional))
            }
            _ => Ok("".to_string()),
        }
    }

    pub fn rn_type_name(
        type_: &impl AsType,
        ci: &ComponentInterface,
    ) -> Result<String, askama::Error> {
        match type_.as_type() {
            Type::Boolean => Ok("Boolean".to_string()),
            Type::Int8 | Type::UInt8 | Type::Int16 | Type::UInt16 | Type::Int32 | Type::UInt32 => {
                Ok("Int".to_string())
            }
            Type::Int64 | Type::UInt64 | Type::Float32 | Type::Float64 => Ok("Double".to_string()),
            Type::String => Ok("String".to_string()),
            Type::Enum { name, .. } => {
                let enum_def = ci.get_enum_definition(&name).unwrap();
                match enum_def.is_flat() {
                    false => Ok("ReadableMap".to_string()),
                    true => Ok("String".to_string()),
                }
            }
            Type::Record { .. } => Ok("ReadableMap".to_string()),
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                rn_type_name(unboxed, ci)
            }
            Type::Sequence { .. } => Ok("ReadableArray".to_string()),
            _ => Ok("".to_string()),
        }
    }

    pub fn temporary(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("{nm}Tmp"))
    }
}

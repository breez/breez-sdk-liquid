use std::collections::HashSet;

use askama::Template;
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToUpperCamelCase};
use once_cell::sync::Lazy;
use uniffi_bindgen::backend::CodeType;
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

// Keywords to fix
static KEYWORDS: Lazy<HashSet<String>> = Lazy::new(|| {
    let list = vec!["Function", "Number", "Object", "Record", "String", "Symbol"];
    HashSet::from_iter(list.into_iter().map(|s| s.to_string()))
});

static IGNORED_FUNCTIONS: Lazy<HashSet<String>> = Lazy::new(|| {
    let list: Vec<&str> = vec!["connect", "add_event_listener", "set_logger"];
    HashSet::from_iter(list.into_iter().map(|s| s.to_string()))
});

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "module.ts")]
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

fn fixup_keyword(name: String, append: String) -> String {
    if KEYWORDS.contains(&name) {
        format!("{name}{append}")
    } else {
        name
    }
}

#[derive(Clone)]
pub struct TypescriptCodeOracle;

impl TypescriptCodeOracle {
    // Map `Type` instances to a `Box<dyn CodeType>` for that type.
    //
    // There is a companion match in `templates/Types.ts` which performs a similar function for the
    // template code.
    //
    //   - When adding additional types here, make sure to also add a match arm to the `Types.ts` template.
    //   - To keep things managable, let's try to limit ourselves to these 2 mega-matches
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
            Type::Duration => {
                unimplemented!("Duration is not implemented")
            }

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
}

impl TypescriptCodeOracle {
    fn find(&self, type_: &impl AsType) -> Box<dyn CodeType> {
        self.create_code_type(type_.as_type())
    }

    /// Get the idiomatic Typescript rendering of a class name (for enums, records, errors, etc).
    fn class_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_upper_camel_case(), "Type".to_string())
    }

    /// Get the idiomatic Typescript rendering of a function name.
    fn fn_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_lower_camel_case(), "Fn".to_string())
    }

    /// Get the idiomatic Typescript rendering of a variable name.
    fn var_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_lower_camel_case(), "Var".to_string())
    }

    /// Get the idiomatic Typescript rendering of an individual enum variant.
    fn enum_variant_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_shouty_snake_case(), "Enum".to_string())
    }
}

pub mod filters {
    use super::*;

    fn oracle() -> &'static TypescriptCodeOracle {
        &TypescriptCodeOracle
    }

    pub fn type_name(type_: &impl AsType) -> Result<String, askama::Error> {
        Ok(oracle().find(type_).type_label())
    }

    /// Get the idiomatic Typescript rendering of a function name.
    pub fn fn_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().fn_name(nm))
    }

    /// Get the idiomatic Typescript rendering of a variable name.
    pub fn var_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().var_name(nm))
    }

    /// Get the idiomatic Typescript rendering of an individual enum variant.
    pub fn enum_variant(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().enum_variant_name(nm))
    }

    pub fn absolute_type_name(type_: &impl AsType) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match type_.as_type() {
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                type_name(unboxed)
            }
            _ => type_name(type_),
        };
        res
    }

    pub fn return_type_name(type_: &impl AsType) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match type_.as_type() {
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                let name = type_name(unboxed)?;
                Ok(format!("{name} | null"))
            }
            _ => type_name(type_),
        };
        res
    }

    pub fn default_value(type_: &impl AsType) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match type_.as_type() {
            Type::Optional { inner_type } => {
                let unboxed = inner_type.as_ref();
                match unboxed {
                    Type::UInt8
                    | Type::Int8
                    | Type::UInt16
                    | Type::Int16
                    | Type::UInt32
                    | Type::Int32
                    | Type::UInt64
                    | Type::Int64
                    | Type::Float32
                    | Type::Float64 => Ok(" = 0".into()),
                    Type::String => Ok(" = \"\"".into()),
                    Type::Record { .. } => Ok(" = {}".into()),
                    Type::Sequence { .. } => Ok(" = []".into()),
                    _ => Ok("".into()),
                }
            }
            _ => Ok("".into()),
        };
        res
    }

    pub fn ignored_function(nm: &str) -> Result<bool, askama::Error> {
        Ok(IGNORED_FUNCTIONS.contains(nm))
    }
}

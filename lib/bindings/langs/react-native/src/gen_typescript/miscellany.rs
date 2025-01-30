use paste::paste;
use uniffi_bindgen::backend::{CodeType, Literal};

macro_rules! impl_code_type_for_miscellany {
    ($T:ty, $canonical_name:literal) => {
        paste! {
            #[derive(Debug)]
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self) -> String {
                    format!("{}", $canonical_name)
                }

                fn canonical_name(&self) -> String {
                    format!("{}", $canonical_name)
                }

                fn literal(&self, _literal: &Literal) -> String {
                    unreachable!()
                }
            }
        }
    };
}

impl_code_type_for_miscellany!(TimestampCodeType, "Date");

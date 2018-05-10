#![recursion_limit = "256"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;
extern crate proc_macro2;

extern crate ident_case;

#[macro_use]
mod macros;

pub mod ast;
pub mod codegen;
pub mod error;
mod from_derive_input;
mod from_field;
mod from_generic_param;
mod from_generics;
mod from_meta_item;
mod from_type_param;
mod from_variant;
pub mod options;
pub mod util;

pub use error::{Error, Result};
pub use from_derive_input::FromDeriveInput;
pub use from_field::FromField;
pub use from_generic_param::FromGenericParam;
pub use from_generics::FromGenerics;
pub use from_meta_item::FromMetaItem;
pub use from_type_param::FromTypeParam;
pub use from_variant::FromVariant;

#![allow(clippy::or_fun_call)]
#![allow(clippy::module_inception)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::await_holding_lock)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

#[macro_use]
extern crate log;
extern crate core;

pub mod api;
pub mod error;

#![cfg(any(target_os = "android", target_os = "ios"))]
#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(improper_ctypes)]
pub mod bindings;
pub use bindings::*;

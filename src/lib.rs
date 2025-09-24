#![feature(

    // Syntax
    decl_macro,

    // Standard library
    maybe_uninit_array_assume_init

)]

pub mod decode;
pub mod encode;
pub mod meta;

pub mod varint;

pub use uuid;
#[cfg(feature = "chrono")]
pub use chrono;

#![doc = include_str!("../README.md")]


#![feature(

    // Syntax
    decl_macro,

    // Standard library
    iter_next_chunk,
    maybe_uninit_array_assume_init

)]

pub mod decode;
pub mod encode;
pub mod meta;

pub mod varint;

pub use uuid;
#[cfg(feature = "chrono")]
pub use chrono;

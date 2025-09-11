#![feature(

    // Syntax
    decl_macro,

    // Standard library
    maybe_uninit_array_assume_init

)]


mod version;
pub use version::Protocol;

pub mod decode;
pub mod encode;
pub mod meta;

pub use uuid;
pub mod varint;

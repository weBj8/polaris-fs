#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
// c2rust transpiles each C TU as a standalone module that redeclares shared
// symbols (fprintf, FILE*, malloc...) against its own local opaque types.
// Same C ABI, different Rust types — inherent to the translation model.
#![allow(clashing_extern_declarations)]

// (macro_use removed: modules import ::c2rust_bitfields directly)
extern crate c2rust_bitfields;

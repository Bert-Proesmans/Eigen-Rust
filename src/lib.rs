// #![deny(missing_docs,
// missing_debug_implementations,
// missing_copy_implementations,
// trivial_casts, trivial_numeric_casts,
// unsafe_code,
// unstable_features,
// unused_import_braces, unused_qualifications)]

#![allow(dead_code)]

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub mod contracts;
pub mod enums;
pub mod core;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#![no_std]
#![warn(missing_docs)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod funcs;
mod types;

pub use funcs::*;
pub use types::*;

#[cfg(test)]
mod tests;

mod integration;
#[allow(unused_imports)]
pub use integration::*;

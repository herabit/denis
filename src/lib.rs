#![allow(unused_unsafe)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod label;

pub(crate) mod util;

#![cfg_attr(feature = "no_std", no_std)]
#![feature(slice_internals)]
#![allow(unused)]

#[cfg(not(feature = "no_std"))]
#[macro_use]
extern crate std;

#[macro_use]
extern crate alloc;

pub mod alignment;
pub mod bitmap;
pub mod buddy_allocator;
pub mod bump_allocator;
pub mod byte_size;
pub mod bytes_parser;
pub mod downcast;
pub mod id_table;
pub mod lazy;
pub mod once;
pub mod ring_buffer;

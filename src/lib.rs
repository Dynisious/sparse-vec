//! Defines a sparse vector container.
//!
//! Last Modified --- 2024-12-30 
//! Author --- DMorgan
#![no_std]
#![deny(missing_docs)]
#![feature(allocator_api,box_vec_non_null,const_vec_string_slice)]

pub use sparse_vecs::SparseVec;

extern crate alloc;

mod bogus_allocs;
mod sparse_vecs;

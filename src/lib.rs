//! Defines a sparse vector container.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03
#![no_std]
#![deny(missing_docs)]
#![feature(allocator_api,box_vec_non_null,const_vec_string_slice)]

pub use sparse_vecs::SparseVec;

extern crate alloc;

mod sparse_vecs;

//! Defines the [BogusAlloc] type.
//!
//! Last Modified --- 2024-12-30 
//! Author --- DMorgan

use alloc::alloc::{Allocator,AllocError,Layout};
use alloc::vec::Vec;
use core::mem;
use core::ptr::NonNull;

/// Bogus Allocator implementation which panics.
pub struct BogusAlloc;

unsafe impl Allocator for BogusAlloc {
  fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    Err(AllocError)
  }
  unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
    unreachable!("no deallocation is allowed")
  }
}

impl BogusAlloc {
  /// Clears `vec` and returns the original allocation managed by `allocator`.
  ///
  /// # Params
  ///
  /// vec --- Vector to clear.  
  /// allocator --- Allocator managing the returned value.  
  ///
  /// # Safety
  ///
  /// `allocator` must be the original allocator of `vec`s memory.
  pub unsafe fn take<T,Alloc>(vec: &mut Vec<T, Self>, allocator: Alloc) -> Vec<T, Alloc>
    where Alloc: Allocator {
    let (vec_ptr,len,capacity) = mem::replace(vec,Vec::new_in(BogusAlloc)).into_parts();

    unsafe { Vec::from_parts_in(vec_ptr,len,capacity,allocator) }
  }
  /// Removes the [Allocator] of `vec` and returns it.
  ///
  /// # Params
  ///
  /// vec --- Vector to take the [Allocator] of.  
  pub fn allocator<T,Alloc>(vec: Vec<T, Alloc>) -> (Vec<T, Self>, Alloc)
    where Alloc: Allocator {
    let (vec_ptr,len,capacity,allocator) = vec.into_parts_with_alloc();
    let vec = unsafe { Vec::from_parts_in(vec_ptr,len,capacity,Self) };

    (vec,allocator)
  }
}

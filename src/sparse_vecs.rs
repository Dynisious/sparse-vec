//! Defines the [SparseVec] type.
//!
//! Last Modified --- 2024-12-30 
//! Author --- DMorgan

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use core::mem;
use core::ops::{Index,IndexMut};
use core::ptr::{self,NonNull};
use crate::bogus_allocs::BogusAlloc;

/// Sparse list of values.
///
/// Maintains separate lists of indices and values.
pub struct SparseVec<T, Alloc>
  where Alloc: Allocator {
  /// External indices of each position in `values`.
  ///
  /// # Invariants
  ///
  /// * Parallel array with `values`.
  /// * Indices are sorted.
  indices: Vec<usize,BogusAlloc>,
  /// Stored values.
  ///
  /// # Invariants
  ///
  /// * Parallel array with `indices`.
  values: Vec<T,BogusAlloc>,
  /// Allocator of the SparseVec.
  allocator: Alloc,
}

impl<T, Alloc> SparseVec<T, Alloc>
  where Alloc: Allocator {
  /// Deconstructs a SparseVec into parts.
  ///
  /// Returns `(Indices: (Pointer, length, capacity), Values: (Pointer, length, capacity), Allocator)`.
  pub fn into_parts(self) -> ((NonNull<usize>, usize, usize), (NonNull<T>, usize, usize), Alloc) {
    let indices = unsafe { ptr::read(&self.indices).into_parts() };
    let values = unsafe { ptr::read(&self.values).into_parts() };
    let allocator = unsafe { ptr::read(&self.allocator) };

    mem::forget(self);
    (indices,values,allocator)
  }
  /// Constructs a SparseVec from parts.
  ///
  /// # Params
  ///
  /// indices --- External indices of each position in `values`.  
  /// values --- Stored values.  
  /// allocator --- Allocator of the SparseVec.  
  ///
  /// # Safety
  ///
  /// * `indices` and `values` must be valid inputs to
  ///     [`Vec::from_parts_in(<variable>,* , allocator)`][Vec::from_parts_in].
  pub unsafe fn from_parts_in(indices: (NonNull<usize>, usize, usize),
                                    values: (NonNull<T>, usize, usize), allocator: Alloc) -> Self {
    let indices = unsafe { Vec::from_parts_in(indices.0,indices.1,indices.2,BogusAlloc) };
    let values = unsafe { Vec::from_parts_in(values.0,values.1,values.2,BogusAlloc) };

    Self{indices,values,allocator}
  }
  /// Constructs an empty SparseVec.
  ///
  /// # Params
  ///
  /// allocator --- Allocator of the SparseVec.  
  pub const fn new_in(allocator: Alloc) -> Self {
    let indices = Vec::new_in(BogusAlloc);
    let values = Vec::new_in(BogusAlloc);

    Self{indices,values,allocator}
  }
  /// Constructs an empty SparseVec with capacity for `capacity` values.
  ///
  /// # Params
  ///
  /// capacity --- Count of values to reserve space for.  
  /// allocator --- Allocator of the SparseVec.  
  pub fn with_capacity_in(capacity: usize, allocator: Alloc) -> Self {
    let indices = BogusAlloc::allocator(Vec::with_capacity_in(capacity,&allocator)).0;
    let values = BogusAlloc::allocator(Vec::with_capacity_in(capacity,&allocator)).0;

    Self{indices,values,allocator}
  }
  /// Returns the number of stored values.
  pub const fn count(&self) -> usize { self.indices.len() }
  /// Tests if `index` holds a value.
  pub fn is_set(&self, index: usize) -> bool {
    self.indices.as_slice().binary_search(&index).is_ok()
  }
  /// Gets the value at `index`.
  ///
  /// Returns `None` if `index` is unset.
  pub fn get(&self, index: usize) -> Option<&T> {
    let value_index = self.indices.binary_search(&index).ok()?;

    Some(unsafe { self.values.get_unchecked(value_index) })
  }
  /// Gets the value at `index`.
  ///
  /// Returns `None` if `index` is unset.
  pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    let value_index = self.indices.binary_search(&index).ok()?;

    Some(unsafe { self.values.get_unchecked_mut(value_index) })
  }
  /// Reserves `space` more positions.
  ///
  /// See [Vec::reserve].
  pub fn reserve(&mut self, space: usize) {
    let mut indices = unsafe { BogusAlloc::take(&mut self.indices,&self.allocator) };

    indices.reserve(space);
    self.indices = BogusAlloc::allocator(indices).0;

    let mut values = unsafe { BogusAlloc::take(&mut self.values,&self.allocator) };

    values.reserve(space);
    self.values = BogusAlloc::allocator(values).0;
  }
  /// Stores `value` at `index` and returns any previously stored value.
  pub fn set(&mut self, index: usize, value: T) -> Option<T> {
    match self.indices.binary_search(&index) {
      Ok(value_index) => Some(mem::replace(&mut self.values[value_index],value)),
      Err(value_index) => {
        self.reserve(1); //Reserve space for the new value.
        self.indices.insert(value_index,index);
        self.values.insert(value_index,value);

        None
      },
    }
  }
  /// Iterates over all set indices.
  pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> + Clone {
    self.indices.iter().copied().zip(self.values.iter())
  }
  /// Iterates over all set indices.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
    self.indices.iter().copied().zip(self.values.iter_mut())
  }
}

impl<T, Alloc> Index<usize> for SparseVec<T, Alloc>
  where Alloc: Allocator {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    self.get(index).expect("accessed and empty index")
  }
}

impl<T, Alloc> IndexMut<usize> for SparseVec<T, Alloc>
  where Alloc: Allocator {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.get_mut(index).expect("accessed and empty index")
  }
}

impl<T, Alloc> Drop for SparseVec<T, Alloc>
  where Alloc: Allocator {
  fn drop(&mut self) {
    unsafe {
      //Take both Vecs and drop them
      let _indices = BogusAlloc::take(&mut self.indices,&self.allocator);
      let _values = BogusAlloc::take(&mut self.values,&self.allocator);
    }
  }
}

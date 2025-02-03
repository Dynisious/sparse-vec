//! Defines the [SparseVec] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use core::mem;
use core::ops::{Index,IndexMut};

/// Sparse list of values.
///
/// Maintains separate lists of indices and values.
#[derive(Clone)]
pub struct SparseVec<T, Alloc>
  where Alloc: Allocator {
  /// External indices of each position in `values`.
  ///
  /// # Invariants
  ///
  /// * Parallel array with `values`.
  /// * Indices are unique.
  /// * Indices are sorted.
  indices: Vec<usize,Alloc>,
  /// Stored values.
  ///
  /// # Invariants
  ///
  /// * Parallel array with `indices`.
  values: Vec<T,Alloc>,
}

impl<T, Alloc> SparseVec<T, Alloc>
  where Alloc: Allocator {
  /// Deconstructs a SparseVec into parts.
  ///
  /// Returns `(Indices, Values)`.
  pub const fn into_parts(self) -> (Vec<usize, Alloc>, Vec<T, Alloc>) {
    use core::ptr;

    let indices = unsafe { ptr::read(&self.indices) };
    let values = unsafe { ptr::read(&self.values) };

    mem::forget(self);
    (indices,values)
  }
  /// Constructs a SparseVec from parts.
  ///
  /// # Params
  ///
  /// indices --- External indices of each position in `values`.  
  /// values --- Stored values.  
  ///
  /// # Safety
  ///
  /// * `indices` must be unique and sorted.  
  pub const unsafe fn from_parts(indices: Vec<usize,Alloc>, values: Vec<T,Alloc>) -> Self {
    Self{indices,values}
  }
  /// Constructs an empty SparseVec.
  ///
  /// # Params
  ///
  /// allocator --- Allocator of the SparseVec.  
  pub fn new_in(allocator: Alloc) -> Self
    where Alloc: Clone {
    let indices = Vec::new_in(allocator.clone());
    let values = Vec::new_in(allocator);

    unsafe { Self::from_parts(indices,values) }
  }
  /// Constructs an empty SparseVec.
  pub fn new() -> Self
    where Alloc: Default {
    let indices = Vec::new_in(Alloc::default());
    let values = Vec::new_in(Alloc::default());

    unsafe { Self::from_parts(indices,values) }
  }
  /// Constructs an empty SparseVec with capacity for `capacity` values.
  ///
  /// # Params
  ///
  /// capacity --- Count of values to reserve space for.  
  /// allocator --- Allocator of the SparseVec.  
  pub fn with_capacity_in(capacity: usize, allocator: Alloc) -> Self
    where Alloc: Clone {
    let indices = Vec::with_capacity_in(capacity,allocator.clone());
    let values = Vec::with_capacity_in(capacity,allocator);

    unsafe { Self::from_parts(indices,values) }
  }
  /// Constructs an empty SparseVec with capacity for `capacity` values.
  ///
  /// # Params
  ///
  /// capacity --- Count of values to reserve space for.  
  pub fn with_capacity(capacity: usize) -> Self
    where Alloc: Default {
    let indices = Vec::with_capacity_in(capacity,Alloc::default());
    let values = Vec::with_capacity_in(capacity,Alloc::default());

    unsafe { Self::from_parts(indices,values) }
  }
  /// Returns the number of stored values.
  pub const fn count(&self) -> usize { self.indices.len() }
  /// Tests is `self` is empty.
  pub const fn is_empty(&self) -> bool { self.indices.is_empty() }
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
    self.indices.reserve(space);
    self.values.reserve(space);
  }
  /// Stores `value` at `index` and returns any previously stored value.
  pub fn set(&mut self, index: usize, value: T) -> Option<T> {
    match self.indices.binary_search(&index) {
      Ok(value_index) => Some(mem::replace(&mut self.values[value_index],value)),
      Err(value_index) => {
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

impl<T,Alloc> Default for SparseVec<T,Alloc>
  where Alloc: Allocator + Default {
  fn default() -> Self { Self::new() }
}

impl<T, Alloc> Index<usize> for SparseVec<T, Alloc>
  where Alloc: Allocator {
  type Output = T;

  #[track_caller]
  fn index(&self, index: usize) -> &Self::Output {
    self.get(index).expect("accessed and empty index")
  }
}

impl<T, Alloc> IndexMut<usize> for SparseVec<T, Alloc>
  where Alloc: Allocator {
  #[track_caller]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.get_mut(index).expect("accessed and empty index")
  }
}

impl<T,Alloc> Eq for SparseVec<T,Alloc>
  where T: Eq, Alloc: Allocator {}

impl<T1,Alloc1,T2,Alloc2> PartialEq<SparseVec<T2,Alloc2>> for SparseVec<T1,Alloc1>
  where T1: PartialEq<T2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &SparseVec<T2,Alloc2>) -> bool {
    self.indices == rhs.indices && self.values == rhs.values
  }
}

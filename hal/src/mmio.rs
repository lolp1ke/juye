// SPDX-License-Identifier: Apache-2.0

use core::{marker::PhantomData, num::NonZero};

#[derive(Clone, Copy)]
pub struct MmioAddr<T> {
  pub addr: NonZero<usize>,
  /// prevents auto impl of [`Send`] and [`Sync`] traits.
  _marker: PhantomData<*mut T>,
}
impl<T> MmioAddr<T> {
  pub const fn new(addr: usize) -> Self {
    Self {
      addr: unsafe { NonZero::new_unchecked(addr) },
      _marker: PhantomData,
    }
  }
  pub const fn as_ptr(self) -> *mut T {
    let addr = self.addr.get();
    addr as *mut T
  }
  pub const fn add(self, offset: usize) -> Self {
    Self::new(self.addr.get() + offset)
  }

  /// # Safety
  ///
  /// - address must be valid and properly alligned (check more in [`core::ptr::write_volatile`]).
  /// - address must point to a readable register.
  pub unsafe fn read(self) -> T {
    unsafe { core::ptr::read_volatile(self.as_ptr()) }
  }

  /// # Safety
  ///
  /// - address must be valid and properly alligned (check more in [`core::ptr::write_volatile`]).
  /// - concurrent access to the same address from other cores are on caller's responsibility to properly syncrhonize.
  pub unsafe fn write(self, value: T) {
    unsafe {
      core::ptr::write_volatile(self.as_ptr(), value);
    }
  }
}

// SPDX-License-Identifier: Apache-2.0

use core::{
  cell::UnsafeCell,
  mem::MaybeUninit,
  sync::atomic::{self, AtomicBool},
};

// Note: swich to &self mb
//       add poison? or just use spin crate
pub struct OnceLock<T> {
  once: AtomicBool,
  value: UnsafeCell<MaybeUninit<T>>,
}
impl<T> OnceLock<T> {
  pub const fn new() -> Self {
    Self {
      once: AtomicBool::new(false),
      value: UnsafeCell::new(MaybeUninit::zeroed()),
    }
  }

  pub fn get_mut_or_init<F>(&mut self, f: F) -> &mut T
  where
    F: FnOnce() -> T,
  {
    self.get_mut_or_try_init(|| Ok::<T, ()>(f())).unwrap()
  }
  pub fn get_mut_or_try_init<F, E>(&mut self, f: F) -> Result<&mut T, E>
  where
    F: FnOnce() -> Result<T, E>,
  {
    if self.get_mut().is_none() {
      self.initialize(f)?;
    };

    Ok(unsafe { self.get_unchecked_mut() })
  }
  pub fn get_or_init<F>(&self, f: F) -> &T
  where
    F: FnOnce() -> T,
  {
    match self.get_or_try_init(|| Ok::<T, core::convert::Infallible>(f())) {
      Ok(value) => value,
    }
  }
  pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
  where
    F: FnOnce() -> Result<T, E>,
  {
    if let Some(value) = self.get() {
      return Ok(value);
    };
    self.initialize(f)?;

    Ok(unsafe { self.get_unchecked() })
  }

  pub fn set(&self, value: T) -> Result<(), T> {
    match self.try_insert(value) {
      Ok(_) => Ok(()),
      Err((_, value)) => Err(value),
    }
  }
  fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
    let mut value = Some(value);
    let res = self.get_or_init(|| value.take().unwrap());
    match value {
      None => Ok(res),
      Some(value) => Err((res, value)),
    }
  }
  fn initialize<F, E>(&self, f: F) -> Result<(), E>
  where
    F: FnOnce() -> Result<T, E>,
  {
    let mut res = Ok(());

    match f() {
      Ok(value) => {
        unsafe {
          (&mut *self.value.get()).write(value);
        };
      }
      Err(err) => res = Err(err),
    };
    res
  }
  const fn initialized(value: T) -> Self {
    Self {
      once: AtomicBool::new(true),
      value: UnsafeCell::new(MaybeUninit::new(value)),
    }
  }

  fn get_mut(&mut self) -> Option<&mut T> {
    if self.once.load(atomic::Ordering::Relaxed) {
      Some(unsafe { self.get_unchecked_mut() })
    } else {
      None
    }
  }
  fn get(&self) -> Option<&T> {
    if self.once.load(atomic::Ordering::Relaxed) {
      Some(unsafe { self.get_unchecked() })
    } else {
      None
    }
  }
  const unsafe fn get_unchecked_mut(&mut self) -> &mut T {
    unsafe { self.value.get_mut().assume_init_mut() }
  }
  const unsafe fn get_unchecked(&self) -> &T {
    unsafe { (&*self.value.get()).assume_init_ref() }
  }
}
impl<T> From<T> for OnceLock<T> {
  fn from(value: T) -> Self {
    Self::initialized(value)
  }
}
impl<T> Default for OnceLock<T> {
  fn default() -> Self {
    Self::new()
  }
}
unsafe impl<T: Send + Sync> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

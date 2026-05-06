// SPDX-License-Identifier: Apache-2.0

use hal::{Platform, uart::Uart};

use crate::get_plarform;

#[macro_export]
macro_rules! print {
  ($($args:tt)*) => {
    $crate::macros::_print(format_args!($($args)*))
  };
}

#[macro_export]
macro_rules! println {
  () => {
    $crate::print!("\r\n");
  };
  ($($args:tt)*) => {
    $crate::print!($($args)*);
    $crate::println!();
  };
}

/// NOTE: Doesn't work with debug args
/// debug requires [`core::fmt::Write`] trait to be implemented
pub(crate) fn _print(args: core::fmt::Arguments<'_>) {
  let uart = get_plarform().uart();

  uart.write_str(args.as_str().unwrap_or("???"));
}

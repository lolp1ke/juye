// SPDX-License-Identifier: Apache-2.0
#![no_std]

pub fn _kstart() -> ! {
  for &b in b"hello world\n" {
    unsafe {
      // hardcoded for test purposes
      core::ptr::write_volatile(0x0900_0000 as *mut u8, b);
    }
  }

  loop {
    unsafe {
      core::arch::asm!("wfe");
    };
  }
}

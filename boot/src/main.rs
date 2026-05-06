// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

#[cfg(target_arch = "aarch64")]
extern crate arch_arm64 as arch;

core::arch::global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub fn _kboot() -> ! {
  kernel::_kstart();
}

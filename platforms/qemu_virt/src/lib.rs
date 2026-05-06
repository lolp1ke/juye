// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hal::Platform;
use uart_driver::plo11::Plo11;

const QEMU_UART_BASE: usize = 0x0900_0000;

pub struct QemuVirtPlatform {
  uart: Plo11,
}
unsafe impl Sync for QemuVirtPlatform {}
unsafe impl Send for QemuVirtPlatform {}

impl Platform for QemuVirtPlatform {
  type Uart = uart_driver::plo11::Plo11;

  fn new() -> Self {
    let uart = Plo11::new(QEMU_UART_BASE);

    Self { uart }
  }

  fn uart(&self) -> &Self::Uart {
    &self.uart
  }
}

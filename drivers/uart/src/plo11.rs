// SPDX-License-Identifier: Apache-2.0

use hal::{mmio::MmioAddr, uart::Uart};

// UART register offsets
// info: https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers?lang=en
const UARTDR: usize = 0x00;
const UARTFR: usize = 0x18;
const UARTFR_TXFF: u32 = 1 << 5;
const UARTIBRD: usize = 0x24;
const UARTFBRD: usize = 0x28;
const UARTLCR_H: usize = 0x2c;
/// [`UARTLCR_H`] flag to enable FIFO
///
/// info: https://developer.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/line-control-register--uartlcr-h?lang=en
const UARTLCR_H_FEN: u32 = 1 << 4;
/// [`UARTLCR_H`] flag to set world len to 8-bits
///
/// info: https://developer.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/line-control-register--uartlcr-h?lang=en
const UARTLCR_H_WLEN: u32 = (1 << 5) | (1 << 6);
const UARTCR: usize = 0x30;
/// [`UARTCR`] flag to enable UART
///
/// info: https://developer.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/control-register--uartcr?lang=en
const UARTCR_UARTEN: u32 = 1 << 0;
/// [`UARTCR`] flag to enable transmit
///
/// info: https://developer.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/control-register--uartcr?lang=en
const UARTCR_TXE: u32 = 1 << 8;
/// [`UARTCR`] flag to enable receive
///
/// info: https://developer.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/control-register--uartcr?lang=en
const UARTCR_RXE: u32 = 1 << 9;
// const UARTIMSC: usize = 0x38;
const UARTICR: usize = 0x44;
/// [`UARTICR`] flags to clear all interrupts
///
/// info: https://developer.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/interrupt-clear-register--uarticr?lang=en
const UARTICR_CLEAR_ALL: u32 = 0b0111_1111_1111;
// const UARTDMACR: usize = 0x48;

const UART_BAUD: u32 = 115200;

pub struct Plo11 {
  base: MmioAddr<u32>,
}
impl Plo11 {
  pub const fn new(base: usize) -> Self {
    Self {
      base: MmioAddr::new(base),
    }
  }
  pub fn init(&self, uartclk: u32) {
    unsafe {
      // disable uart
      #[expect(
        clippy::erasing_op,
        reason = "& 0 indicates OFF state of [`UARTCR_UARTEN`] flag"
      )]
      self.base.add(UARTCR).write(const { UARTCR_UARTEN & 0 });

      // clear interrupts
      self.base.add(UARTICR).write(UARTICR_CLEAR_ALL);

      // baud rate
      let (ibrd, fbrd) = calc_baud(uartclk, UART_BAUD);
      self.base.add(UARTIBRD).write(ibrd);
      self.base.add(UARTFBRD).write(fbrd);

      // enable FIFO, set 8-bits world len
      self
        .base
        .add(UARTLCR_H)
        .write(UARTLCR_H_FEN | UARTLCR_H_WLEN);

      // enable UART, TX, RX
      self
        .base
        .add(UARTCR)
        .write(UARTCR_UARTEN | UARTCR_TXE | UARTCR_RXE);
    };
  }

  #[inline(always)]
  fn write_byte(&self, byte: u8) {
    unsafe {
      // wait if queue is full
      while self.base.add(UARTFR).read() & UARTFR_TXFF != 0 {
        core::hint::spin_loop();
      }
      self.base.add(UARTDR).write(byte as u32);
    }
  }
}
impl Uart for Plo11 {
  fn write_byte(&self, byte: u8) {
    if core::hint::unlikely(byte == b'\n') {
      self.write_byte(b'\r');
      self.write_byte(b'\n');
      return;
    };

    self.write_byte(byte);
  }
}

#[inline]
fn calc_baud(uartclk: u32, baud: u32) -> (u32, u32) {
  let divisor = uartclk as f64 / (16.0 * baud as f64);
  let ibrd = libm::floor(divisor) as u32;
  let fbrd = libm::round((divisor - ibrd as f64) * 64.0) as u32;
  (ibrd, fbrd)
}

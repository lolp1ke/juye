// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(not(any(feature = "qemu_virt")))]
compile_error!("At least one platform must be selected");

use hal::Platform;
use kcore::once_lock::OnceLock;

#[cfg(feature = "qemu_virt")]
use qemu_virt_platform::QemuVirtPlatform as RealPlatform;

// #[cfg(feature = "<platform>")]
// use <plaform>_platform::<platform>Platform as RealPlatform;

#[macro_use]
mod macros;

pub fn _kstart() -> ! {
  let platform = get_plarform();
  let uart = platform.uart();

  // TODO: read from dtb
  uart.init(24_000_000);

  println!("kernel booted");

  loop {
    unsafe {
      core::arch::asm!("wfe");
    };
  }
}

static PLATFORM: OnceLock<RealPlatform> = OnceLock::new();
pub(crate) fn get_plarform() -> &'static RealPlatform {
  PLATFORM.get_or_init(<RealPlatform as Platform>::new)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  use hal::uart::Uart;
  let uart = get_plarform().uart();

  uart.write_str("KERNEL PANIC\n");
  let loc = info.location();

  if let Some(loc) = loc {
    uart.write_str("at ");
    uart.write_str(loc.file());
    uart.write_str(":");
    write_dec(uart, loc.line() as u64);
    uart.write_str(":");
    write_dec(uart, loc.column() as u64);
    uart.write_str("\n");
  };
  uart.write_str(info.message().as_str().unwrap_or("???"));
  uart.write_str("\n");

  loop {
    core::hint::spin_loop();
  }

  #[inline(always)]
  fn write_dec<S: hal::uart::Uart>(uart: &S, mut n: u64) {
    const BUF_SIZE: usize = 256;
    let mut buf = [0u8; BUF_SIZE];
    let mut i = BUF_SIZE;
    if n == 0 {
      uart.write_byte(b'0');
      return;
    }
    while n > 0 {
      i -= 1;
      buf[i] = b'0' + (n % 10) as u8;
      n /= 10;
    }
    uart.write_str(core::str::from_utf8(&buf[i..]).unwrap_or("???"));
  }
}

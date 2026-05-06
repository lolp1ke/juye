// SPDX-License-Identifier: Apache-2.0

// NOTE: &mut self needed for Write trait
//       Write trait is essential for debugging purposes (can be discarded thou but much easier to debug with it as it unlocks format_args!)
pub trait Uart: 'static {
  fn write_byte(&self, byte: u8);
  fn write_str(&self, s: &str) {
    for byte in s.bytes() {
      self.write_byte(byte);
    }
  }
}

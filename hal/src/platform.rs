// SPDX-License-Identifier: Apache-2.0

use crate::uart;

// NOTE: planning to migrate from &self to &mut self for Write trait
//       or wrap types in some RwLock
pub trait Platform: 'static {
  type Uart: uart::Uart;

  fn new() -> Self;

  fn uart(&self) -> &Self::Uart;
}

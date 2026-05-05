/* SPDX-License-Identifier: Apache-2.0 */

.section .text.kboot
.global _start

_start:
  // park all cores expect for one
  // this will eliminate multiple _kboot calls
  mrs x0, mpidr_el1
  and x0, x0, #0xFF
  cbnz x0, .park

  // decrease privillage level from el2 -> el1
  // link to refresh memory https://developer.arm.com/documentation/102412/0103/Privilege-and-Exception-levels/Exception-levels
  mrs x0, CurrentEL
  lsr x0, x0, #2
  cmp x0, #2
  // already in el1
  bne .in_el1

  // set el1's execution env 64-bit
  // 31's bit is RW which indicates wether it is AArch32/AArch64
  // https://developer.arm.com/documentation/111107/2026-03/AArch64-Registers/HCR-EL2--Hypervisor-Configuration-Register?lang=en
  mov x0, #(1 << 31)
  msr hcr_el2, x0

  // all exceptions mask
  // resources
  // https://developer.arm.com/documentation/PRD29-GENC-009492/c/TrustZone-Hardware-Architecture/Processor-architecture/Secure-interrupts
  // https://developer.arm.com/documentation/ddi0601/2026-03/AArch64-Registers/SPSR-EL2--Saved-Program-Status-Register--EL2-?lang=en
  // 0000_0011_1100_0100
  mov x0, #0x3C4
  msr spsr_el2, x0

  // jump to .in_el1 after leaving el2 execution env
  adr x0, .in_el1
  msr elr_el2, x0
  eret

.in_el1:
  adr x0, __bss_start
  adr x1, __bss_end
.zero_bss:
  cmp x0, x1
  bge .bss_done
  str xzr, [x0], #8
  b .zero_bss
.bss_done:
  adr x0, __boot_stack_top
  mov sp, x0

  // call rust entry point
  // boot/src/main.rs
  bl _kboot

.halt:
  wfe
  b .halt
.park:
  wfe
  b .park

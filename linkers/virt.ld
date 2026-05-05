/* SPDX-License-Identifier: Apache-2.0 */

ENTRY(_start)
OUTPUT_FORMAT("elf64-littleaarch64", "elf64-littleaarch64", "elf64-littleaarch64")

SECTIONS {
  /* qemu's entry point */
  . = 0x40000000;

  .text : {
    /* do not accidentally optimize necessary code */
    KEEP(*(.text.kboot))
    *(.text .text.*)
  }
  .rodata : {
    *(.rodata .rodata.*)
  }
  .data : {
    *(.data .data.*)
  }
  .bss (NOLOAD) : {
    __bss_start = .;
    *(.bss .bss.*)
    __bss_end = .;
  }

  /* align to 16 bytes for sp */
  /* reserve 16KiB for boot stack */
  . = ALIGN(16);
  . += 0x4000;
  __boot_stack_top = .;
}

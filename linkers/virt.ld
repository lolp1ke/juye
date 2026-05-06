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
  
  . = ALIGN(4096);
  .rodata : {
    *(.rodata .rodata.*)
  }
  . = ALIGN(4096);
  .data : {
    *(.data .data.*)
  }
  . = ALIGN(4096);
  .bss : {
    __bss_start = .;
    *(.bss .bss.*)
    __bss_end = .;
  }

  /* align to 4KiB for sp */
  /* reserve 32KiB for boot stack */
  . = ALIGN(4096);
  __boot_stack_top = . + 0x8000;

  /DISCARD/ : {
    *(.eh_frame*)
    *(.ARM.exidx*)
    *(.note*)
    *(.comment*)
  }
}

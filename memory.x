/* stm32f405rgt6 */
MEMORY {
  FLASH : ORIGIN   = 0x08000000, LENGTH = 1M
  CCM_RAM : ORIGIN = 0x10000000, LENGTH = 64K
  RAM : ORIGIN     = 0x20000000, LENGTH = 128K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

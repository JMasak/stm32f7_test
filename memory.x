MEMORY

{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM   : ORIGIN = 0x20000000, LENGTH = 320K  
}

/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Size of heap (in bytes) /*
/* _heap_size = 1024; */

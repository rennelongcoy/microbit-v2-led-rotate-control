/* Linker script for the nRF52833 */
/*
  4.2 Memory
    The nRF52833 contains 512 kB of flash memory and 128 kB of RAM that can be used for code and data
    storage.

    From Figure 3: Memory Map, the start addresses are:
      Flash: 0x00000000
      Data RAM: 0x20000000
*/
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}

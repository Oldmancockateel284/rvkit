/* ESP32-C3 linker script - placeholder */
ENTRY(_start)

MEMORY
{
    FLASH (rx) : ORIGIN = 0x42000000, LENGTH = 4M
    RAM (rwx)  : ORIGIN = 0x3FC80000, LENGTH = 400K
}

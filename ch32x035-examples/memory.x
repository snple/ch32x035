/* Memory layout for the CH32X035 (WCH QingKe V4C)
 *
 * The CH32X035 has 62K flash (0x00000000) and 20K SRAM (0x20000000).
 * Both regions are used by the ch32x035-rt linker script (link.x).
 */

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 62K
    RAM   (xrw) : ORIGIN = 0x20000000, LENGTH = 20K
}

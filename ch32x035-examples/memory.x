/* Memory layout for the CH32X035 (WCH QingKe V4C)
 *
 * The CH32X035 has 62K flash (0x00000000) and 20K SRAM (0x20000000).
 * The WCH QingKe hardware stack (used automatically for interrupt/exception
 * frame saves) occupies the very bottom of SRAM (0x20000000..0x20000400,
 * 1 KiB), so the linker RAM region starts above it to keep .data/.bss from
 * being overwritten by hardware-stack pushes.
 */

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 62K
    RAM   (xrw) : ORIGIN = 0x20000400, LENGTH = 19K
}

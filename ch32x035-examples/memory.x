/* Memory layout for the CH32X035 (WCH QingKe V4C)
 *
 * The CH32X035 has 62K flash (0x00000000) and 20K SRAM (0x20000000).
 *
 * The QingKe V4 hardware stack (HPE, see QingKeV4 Processor Manual §3.4) saves
 * the 16 caller-saved integer registers into an *internal* stack area that is
 * invisible to software — it does NOT occupy SRAM. Only when the 3-level
 * hardware stack overflows (or FPU registers must be saved) does the core fall
 * back to the user stack in RAM. Therefore the full 20K SRAM is available for
 * .data/.bss and the software stack.
 */

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 62K
    RAM   (xrw) : ORIGIN = 0x20000000, LENGTH = 20K
}

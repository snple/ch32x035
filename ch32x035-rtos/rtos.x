/* ch32x035-rtos linker script fragment.
 *
 * `#[task]` places one `TaskDef` per task into `.rtos.tasks`; `rtos::start()`
 * walks `__rtos_tasks_start`..`__rtos_tasks_end` to build the task table.
 *
 * Linked via `-Trtos.x` by any binary that uses `ch32x035-rtos` (see
 * `ch32x035-examples/build.rs`). `INSERT AFTER .vector_table` places the
 * table right after the WCH vector table in flash, without depending on the
 * region definition order of the other `-T` scripts.
 */
SECTIONS {
    .rtos.tasks : ALIGN(4) {
        __rtos_tasks_start = .;
        KEEP(*(.rtos.tasks));
        __rtos_tasks_end = .;
    }
} INSERT AFTER .vector_table;

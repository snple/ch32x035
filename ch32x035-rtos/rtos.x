/* ch32x035-rtos 链接脚本片段。
 *
 * `#[task]` 把每个 `TaskDef` 放入 `.rtos.tasks`；`rtos::start()` 遍历
 * `__rtos_tasks_start`..`__rtos_tasks_end` 构建任务表。
 *
 * 由使用内核的二进制通过 `-Trtos.x` 链接（见 ch32x035-examples/build.rs）。
 * `INSERT AFTER .vector_table` 把任务表放在向量表之后，不依赖其它 `-T` 脚本的
 * 区域定义顺序。
 */
SECTIONS {
    .rtos.tasks : ALIGN(4) {
        __rtos_tasks_start = .;
        KEEP(*(.rtos.tasks));
        __rtos_tasks_end = .;
    }
} INSERT AFTER .vector_table;

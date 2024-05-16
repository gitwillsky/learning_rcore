    .section .text.entry            # 定义代码段 entry
    .globl   _start                 # 定义全局符号
_start:
    la       sp, boot_stack_top
    call     rust_main

    .section .bss.stack
    .globl   boot_stack_lower_bound
boot_stack_lower_bound:
    .space   4096 * 16
    .globl   boot_stack_top
boot_stack_top:
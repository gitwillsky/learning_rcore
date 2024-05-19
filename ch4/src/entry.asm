    .section .text.entry            # 定义代码段 entry
    .globl   _start                 # 定义全局符号
_start:
    la       sp, boot_stack_top     # 将 boot_stack_top 地址存入栈指针
    call     rust_main              # 调用 rust_main

    .section .bss.stack             # 定义未初始化的 stack 全局变量
    .globl   boot_stack_lower_bound # 定义全局符号 boot_stack_lower_bound
boot_stack_lower_bound:
    .space   4096 * 16              # 分配 16kb 内存
    .globl   boot_stack_top         # 定义全局符号 boot_stack_top
boot_stack_top:
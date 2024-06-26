
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 11
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_8_start
    .quad app_9_start
    .quad app_10_start
    .quad app_10_end

    .global _app_names
    _app_names:
    
    .string "exit"
    .string "fork"
    .string "hello_world"
    .string "initproc"
    .string "power_3"
    .string "priv_inst"
    .string "sbrk_test"
    .string "sleep"
    .string "stackoverflow"
    .string "store_fault"
    .string "user_shell"

    .section .data
    .global app_0_start
    .global app_0_end
    .align 3
app_0_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/exit"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 3
app_1_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/fork"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .align 3
app_2_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/hello_world"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
    .align 3
app_3_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/initproc"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
    .align 3
app_4_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/power_3"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
    .align 3
app_5_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/priv_inst"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
    .align 3
app_6_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/sbrk_test"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
    .align 3
app_7_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/sleep"
app_7_end:

    .section .data
    .global app_8_start
    .global app_8_end
    .align 3
app_8_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/stackoverflow"
app_8_end:

    .section .data
    .global app_9_start
    .global app_9_end
    .align 3
app_9_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/store_fault"
app_9_end:

    .section .data
    .global app_10_start
    .global app_10_end
    .align 3
app_10_start:
    .incbin "../ch5_user/target/riscv64gc-unknown-none-elf/release/user_shell"
app_10_end:

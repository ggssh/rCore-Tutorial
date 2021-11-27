    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top # 将sp设置为预留的栈空间的栈顶的位置
    call rust_main

# 这块栈空间单独作为一个名为.bss.stack的段
    .section .bss.stack
    # 栈底
    .globl boot_stack
boot_stack: 
    .space 4096 * 16
    # 栈顶
    .globl boot_stack_top
boot_stack_top: 
    
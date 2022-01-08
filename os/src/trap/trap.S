.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

# 在执行__alltraps或__restore函数进行地址空间切换时
# 应用的用户态虚拟地址空间对切换地址空间的指令所在页的映射方式是相同的
# 说明这段切换地址空间的指令控制流仍是可以连续执行的
    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2 // 将__alltraps地址4字节对齐
__alltraps:
    # sp 和 sscratch交换
    csrrw sp, sscratch, sp
    # now sp->*TrapContext in user space, sscratch->user stack
    # save other general purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack(csrr 读控制状态寄存器 csrr rd,csr x[rd]=CSRs[csr])
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # load kernel_stap into t0
    ld t0, 34*8(sp)
    # load trap_handler into t1
    ld t1, 36*8(sp)
    # move to kernel_sp
    ld sp, 35*8(sp)
    # 将satp修改为内核地址空间的token并使用sfence.vma刷新快表，切换到内核地址空间
    csrw satp, t0
    sfence.vma
    # jump to trap_handler(不能像之前一样call TrapHandler)
    # 跳转指令实际被执行时的虚拟地址和在编译器/汇编器/链接器进行后端代码生成和链接形成最终机器码时设置此指令的地址是不同的
    jr t1

__restore:
    # 在 __switch 之后，sp 就已经正确指向了我们需要的 Trap 上下文地址
    # mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc

    # a0: *TrapContext in user space(Constant)Trap上下文在应用地址空间中的位置，对于所有的应用来说都是相同的
    # a1: user space token 即将返回到的应用的地址空间的token
    # switch to user space
    csrw satp, a1
    sfence.vma
    csrw sscratch, a0
    mv sp,a0
    #now sp points to TrapContext in user space, start restoring based on it
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general-purpuse registers except x0/sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # back to user stack
    ld sp, 2*8(sp)
    sret

OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

/*comments*/
SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
        
        /*
            上述链接指令等价于 
            *(.text)
            *(.text.*)
        */
    }
    . = ALIGN(4K);
    etext = .;

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    erodata = .;

    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;

    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    ebss = .;

    ekernel = .;

    /*
        被该section引用的任何输入section将不会出现在输出文件中
    */
    /DISCARD/ : {
        *(.eh_frame)
    }
}
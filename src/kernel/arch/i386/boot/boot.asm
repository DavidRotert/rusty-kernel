# INTEL SYNTAX

.set MAGIC, 0x1BADB002
.set FLAGS, 0
.set CHECKSUM, -(MAGIC + FLAGS)

.section .multiboot

.long MAGIC
.long FLAGS
.long CHECKSUM

stackBottom:
# set the maximum size of stack
.skip 32 * 1024


# set the stack top which grows from higher to lower
stackTop:

.section .text

.global _start
.type _start, @function
_start:
    # assign current stack pointer location to stackTop
    mov $stackTop, %esp

    # call the kernel main source
    call kernel_main

    cli

# put system in infinite loop
hltLoop:
    hlt
    jmp hltLoop

.size _start, . - _start

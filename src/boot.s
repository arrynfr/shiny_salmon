.section .text._start
.global _start
.type _start, @function
_preamble:
    adrp x0, _base
    adrp x1, _rela_start
    add x1, x1, :lo12:_rela_start
    adrp x2, _rela_end
    add x2, x2, :lo12:_rela_end
    bl _relocate_binary

_start:
    #Disable interrupts
    msr DAIFSet, #0b1111
    isb
    
    #Set up exception vectors
    adr x7, vector_table_el1
    msr vbar_el1, x7

    #Enable floating point bits FPEN
    mrs x7, cpacr_el1
    mov x8, #(3 << 20)
    orr x7, x8, x7
    msr cpacr_el1, x7
    
    #Store core number in tpidr
    mrs x9, mpidr_el1
    and x9, x9, 0xFF
    msr tpidr_el1, x9
    
    #Set up stack
    adr x7, {}
    mov sp, x7
    mov x8, {}
    mul x8, x8, x9
    add sp, sp, x8
    
    bl clear_bss
    bl main
    b .

# in:   (x0 = base, x1 = rela_start, x2 = rela_end)
# mod:  (x12 = binary_address, x13 = addend_address,
#               x9 = offset, x10 = type, x11 = addend)
.equ R_AARCH64_RELATIVE, 1027
.global _relocate_binary
.type _relocate_binary, @function
_relocate_binary:
    ldp x9, x10, [x1], 0x10
    ldr x11, [x1], 0x8

    cmp x10, R_AARCH64_RELATIVE
    bne 1f

    add x12, x0, x9
    add x13, x0, x11
    str x13, [x12]
    cmp x1, x2
    bne _relocate_binary
1:
    ret

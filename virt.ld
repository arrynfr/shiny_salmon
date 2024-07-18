ENTRY(_preamble)
_alignment = 0x10000;

/*. = 0x40800000;*/
. = 0;

PHDRS {
	segment_text	PT_LOAD FLAGS(5);
	segment_data	PT_LOAD FLAGS(6);
	segment_rodata	PT_LOAD FLAGS(4);
}

SECTIONS {
	_base = .;
	.text : ALIGN(_alignment) {
		KEEP(*(.text._start))
		*(.text*)
	} :segment_text
	.data : ALIGN(_alignment) {
		*(.data*)
	} :segment_data
	.rodata : ALIGN(_alignment) {
		*(.rodata*)
	} :segment_rodata
	    .rela.dyn : ALIGN(_alignment) {
        _rela_start = .;
        *(.rela.*)
        _rela_end = .;
    } :segment_rodata
	.bss (NOLOAD) : ALIGN(_alignment) {
		_bss_start = .;
		*(.sbss .sbss.*)
		*(.bss .bss.*)
		. = ALIGN(16);
		_bss_end = .;
	}
	.stack : ALIGN(_alignment) {
		KEEP(*(.stack*))
	}
	_end = .;
}

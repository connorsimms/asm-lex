	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main
	.p2align	2
_main:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	adrp	x0, l_.str@PAGE
	add	x0, x0, l_.str@PAGEOFF
	bl	_puts
	; InlineAsm Start
	nop
	; InlineAsm End
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.section	__TEXT,__cstring,cstring_literals
l_.str:
	.asciz	"Hello World!\n"

.subsections_via_symbols

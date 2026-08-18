	.file	"hello-world.c"
	.text
	.globl	main
	.p2align	2
	.type	main,@function
main:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	adrp	x0, .L.str
	add	x0, x0, :lo12:.L.str
	bl	puts
	mov	w0, wzr
	ldp	x29, x30, [sp], #16
	ret
.Lfunc_end0:
	.size	main, .Lfunc_end0-main

	.type	.L.str,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.L.str:
	.asciz	"Hello World!\n"
	.size	.L.str, 14

	.ident	"<toolchain>"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym puts

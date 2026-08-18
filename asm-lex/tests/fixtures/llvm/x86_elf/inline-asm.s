	.file	"hello-world-inline-asm.c"
	.text
	.globl	main
	.p2align	4
	.type	main,@function
main:
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	.L.str(%rip), %rdi
	callq	puts@PLT
	#APP
	nop
	#NO_APP
	xorl	%eax, %eax
	popq	%rbp
	retq
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

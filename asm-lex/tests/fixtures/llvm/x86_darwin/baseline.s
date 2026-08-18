	.macosx_version_min 10, 4
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main
	.p2align	4
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	L_.str(%rip), %rdi
	callq	_puts
	xorl	%eax, %eax
	popq	%rbp
	retq

	.section	__TEXT,__cstring,cstring_literals
L_.str:
	.asciz	"Hello World!\n"

.subsections_via_symbols

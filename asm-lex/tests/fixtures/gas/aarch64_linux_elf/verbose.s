	.arch armv8-a
	.file	"hello-world.c"
// GNU C23 (GCC) version 15.2.0 (aarch64-unknown-linux-gnu)
//	compiled by GNU C version 15.2.0, GMP version 6.3.0, MPFR version 4.2.2, MPC version 1.4.1, isl version isl-0.20-GMP

// GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
// options passed: -mno-omit-leaf-frame-pointer -march=armv8-a -mlittle-endian -mabi=lp64 -O0 -fno-omit-frame-pointer -ffreestanding -frandom-seed=0
	.text
	.section	.rodata
	.align	3
.LC0:
	.string	"Hello World!\n"
	.text
	.align	2
	.global	main
	.type	main, %function
main:
.LFB0:
	.cfi_startproc
	stp	x29, x30, [sp, -16]!	//,,,
	.cfi_def_cfa_offset 16
	.cfi_offset 29, -16
	.cfi_offset 30, -8
	mov	x29, sp	//,
// hello-world.c:4:   puts("Hello World!\n");
	adrp	x0, .LC0	// tmp103,
	add	x0, x0, :lo12:.LC0	//, tmp103,
	bl	puts		//
// hello-world.c:5:   return 0;
	mov	w0, 0	// _3,
// hello-world.c:6: }
	ldp	x29, x30, [sp], 16	//,,,
	.cfi_restore 30
	.cfi_restore 29
	.cfi_def_cfa_offset 0
	ret	
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.ident	"<toolchain>"
	.section	.note.GNU-stack,"",@progbits

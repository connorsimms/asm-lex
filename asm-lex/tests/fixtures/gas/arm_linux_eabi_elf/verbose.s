	.arch armv7-a
	.fpu vfpv3-d16
	.eabi_attribute 28, 1	@ Tag_ABI_VFP_args
	.eabi_attribute 20, 1	@ Tag_ABI_FP_denormal
	.eabi_attribute 21, 1	@ Tag_ABI_FP_exceptions
	.eabi_attribute 23, 3	@ Tag_ABI_FP_number_model
	.eabi_attribute 24, 1	@ Tag_ABI_align8_needed
	.eabi_attribute 25, 1	@ Tag_ABI_align8_preserved
	.eabi_attribute 26, 2	@ Tag_ABI_enum_size
	.eabi_attribute 30, 6	@ Tag_ABI_optimization_goals
	.eabi_attribute 34, 1	@ Tag_CPU_unaligned_access
	.eabi_attribute 18, 4	@ Tag_ABI_PCS_wchar_t
	.file	"hello-world.c"
@ GNU C23 (GCC) version 15.2.0 (armv7l-unknown-linux-gnueabihf)
@	compiled by GNU C version 15.2.0, GMP version 6.3.0, MPFR version 4.2.2, MPC version 1.4.1, isl version isl-0.20-GMP

@ GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
@ options passed: -mfpu=vfpv3-d16 -mfloat-abi=hard -mtls-dialect=gnu -marm -march=armv7-a+fp -O0 -fno-omit-frame-pointer -ffreestanding -frandom-seed=0
	.text
	.section	.rodata
	.align	2
.LC0:
	.ascii	"Hello World!\012\000"
	.text
	.align	2
	.global	main
	.syntax unified
	.arm
	.type	main, %function
main:
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 1, uses_anonymous_args = 0
	push	{fp, lr}	@
	add	fp, sp, #4	@,,
@ hello-world.c:4:   puts("Hello World!\n");
	ldr	r3, .L3	@ tmp116,
.LPIC0:
	add	r3, pc, r3	@ tmp116, tmp116
	mov	r0, r3	@, tmp116
	bl	puts(PLT)	@
@ hello-world.c:5:   return 0;
	mov	r3, #0	@ _3,
@ hello-world.c:6: }
	mov	r0, r3	@, <retval>
	pop	{fp, pc}	@
.L4:
	.align	2
.L3:
	.word	.LC0-(.LPIC0+8)
	.size	main, .-main
	.ident	"<toolchain>"
	.section	.note.GNU-stack,"",%progbits

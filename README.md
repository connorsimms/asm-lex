# asm-lex

A lexer for assembly source in the GNU assembler and LLVM `llvm-mc` dialects.

Compiler-generated assembly can look very different depending on a variety of factors (CPU, object format, ...). 

For example, `#` starts a comment on x86 ELF but not on ARM, where `@` does. `$` may or may not begin an identifier. `//` is a comment on some targets and a division operator on others. The list goes on.

Tools that want to read or analyze assembly from more than one compiler usually end up with a mess of per-target special cases, or with a catch-all regular expression that is wrong for many edge cases.

`asm-lex` aims to move these idiosyncrasies into the type system. Targets are configured by a set of associated constants on a trait, and the lexer is generic over it.

```rust
use asm_lex::source::gas::{targets::X86LinuxElf, Gas};
use asm_lex::source::lexer::Lexer;
use asm_lex::source::Kind;
 
let src = b"main:\n    movq %rsp, %rbp  # prologue\n";
 
for item in Lexer::<Gas<X86LinuxElf>>::new(src) {
    match item.kind() {
        Kind::Label { name } => println!("label {}", String::from_utf8_lossy(&src[name.clone()])),
        Kind::Instruction { mnemonic, .. } => {
            println!("insn  {}", String::from_utf8_lossy(&src[mnemonic.clone()]))
        }
        _ => {}
    }
}
```
Swap `Gas<X86LinuxElf>` for `Llvm<Aarch64Darwin>` and the loop above is unchanged.

## The data model

| Kind | Fields |
| --- | --- |
| `Label` | `name` |
| `Directive` | `name`, `args` |
| `Instruction` | `mnemonic`, `args` |
| `Definition` | `symbol`, `keyword`, `args` |
| `Comment` | |
| `Preprocessor` | |
| `Unknown` | |

Every field is a `Range<usize>` or `Option<Range<usize>>` into the input. 
Nothing is copied or allocated as the lexer only holds slices and offsets.
Malformed input does not produce errors. It produces `Unknown` instead.

## Targets

**GNU assembler** — `X86GenericElf`, `X86LinuxElf`, `X86Darwin`, `X86Pe`,
`Aarch64GenericElf`, `Aarch64LinuxElf`, `Aarch64Pe`, `ArmGenericElf`, `ArmLinuxElf`,
`ArmLinuxEabiElf`, `ArmPe`, `RiscvGenericElf`

**LLVM** — `X86Elf`, `X86Darwin`, `X86Microsoft`, `X86GnuCoff`, `Aarch64Elf`,
`Aarch64Darwin`, `Aarch64MicrosoftCoff`, `Aarch64GnuCoff`, `ArmElf`, `ArmDarwin`,
`ArmMicrosoftCoff`, `ArmGnuCoff`, `RiscvElf`, `RiscvDarwin`

## Benchmarks

### Approach
Benchmarks use the SQLite amalgamation compiled to assembly four ways:
`-O0`, `-O2`, `-O2 -fverbose-asm`, and `-O0 -g`, then truncated at line boundaries to
each size. `cargo xtask bench` downloads the source, verifies its SHA3-256, and
generates all fixtures.

### Results
Criterion median, AMD Ryzen 9 5950X, rustc 1.97.1:

| | 10 KiB | 100 KiB | 1 MiB |
| --- | --- | --- | --- |
| **gas** `-O0` | 1049 MiB/s | 876 MiB/s | 874 MiB/s |
| **gas** `-O2` | 899 MiB/s | 809 MiB/s | 781 MiB/s |
| **gas** `-O2 -fverbose-asm` | 1072 MiB/s | 1010 MiB/s | 1001 MiB/s |
| **gas** `-O0 -g` | 1038 MiB/s | 863 MiB/s | 848 MiB/s |
| **llvm** `-O0` | 728 MiB/s | 685 MiB/s | 692 MiB/s |
| **llvm** `-O2` | 649 MiB/s | 620 MiB/s | 622 MiB/s |
| **llvm** `-O2 -fverbose-asm` | 739 MiB/s | 763 MiB/s | 728 MiB/s |
| **llvm** `-O0 -g` | 814 MiB/s | 709 MiB/s | 713 MiB/s |

## Status

Pre-1.0 and the API will (most likely) change. 

Module `listing`, for reading assembler listing output (objdump, llvm-objdump) is not yet implemented.

MSRV is 1.70.0.

mod common;

use asm_lex::source::gas::targets::*;
use asm_lex::source::gas::*;
use common::snapshot;

#[test]
fn x86_64_linux_elf() {
    insta::glob!("fixtures/gas/x86_64_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = snapshot::SourceLexer::<Gas<X86_64LinuxElf>>::from_bytes(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer.collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn aarch64_linux_elf() {
    insta::glob!("fixtures/gas/aarch64_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = snapshot::SourceLexer::<Gas<Aarch64LinuxElf>>::from_bytes(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer.collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn arm_linux_eabi_elf() {
    insta::glob!("fixtures/gas/arm_linux_eabi_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = snapshot::SourceLexer::<Gas<ArmLinuxEabiElf>>::from_bytes(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer.collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn riscv64_linux_elf() {
    insta::glob!("fixtures/gas/riscv64_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = snapshot::SourceLexer::<Gas<Riscv64LinuxElf>>::from_bytes(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer.collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

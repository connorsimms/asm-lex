#![allow(clippy::incompatible_msrv)]

mod common;

use asm_lex::source::gas::{targets::*, *};
use asm_lex::source::lexer::Lexer;
use common::snapshot;

#[test]
fn x86_linux_elf() {
    insta::glob!("fixtures/gas/x86_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Gas<X86LinuxElf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn aarch64_linux_elf() {
    insta::glob!("fixtures/gas/aarch64_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Gas<Aarch64LinuxElf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn arm_linux_eabi_elf() {
    insta::glob!("fixtures/gas/arm_linux_eabi_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Gas<ArmLinuxEabiElf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn riscv_elf() {
    insta::glob!("fixtures/gas/riscv_generic_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Gas<RiscvGenericElf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

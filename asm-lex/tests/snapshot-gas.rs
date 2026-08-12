#![allow(clippy::incompatible_msrv)]

mod common;

use asm_lex::source::gas::{targets::*, *};
use asm_lex::source::lexer::Lexer;
use common::snapshot;

#[test]
fn x86_64_linux_elf() {
    insta::glob!("fixtures/gas/x86_64_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Gas<X86_64LinuxElf>>::new(&bytes);
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
fn riscv64_linux_elf() {
    insta::glob!("fixtures/gas/riscv64_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Gas<Riscv64LinuxElf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

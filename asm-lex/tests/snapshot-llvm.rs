#![allow(clippy::incompatible_msrv)]

mod common;

use asm_lex::source::lexer::Lexer;
use asm_lex::source::llvm::{targets::*, Llvm};
use common::snapshot;

#[test]
fn aarch64_darwin() {
    insta::glob!("fixtures/llvm/aarch64_darwin/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Llvm<Aarch64Darwin>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn aarch64_elf() {
    insta::glob!("fixtures/llvm/aarch64_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Llvm<Aarch64Elf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn arm_elf() {
    insta::glob!("fixtures/llvm/arm_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Llvm<ArmElf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn riscv_elf() {
    insta::glob!("fixtures/llvm/riscv_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Llvm<RiscvElf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn x86_darwin() {
    insta::glob!("fixtures/llvm/x86_darwin/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Llvm<X86Darwin>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

#[test]
fn x86_elf() {
    insta::glob!("fixtures/llvm/x86_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = Lexer::<Llvm<X86Elf>>::new(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer
            .map(|item| snapshot::Item::from_source_item(&item, &bytes))
            .collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

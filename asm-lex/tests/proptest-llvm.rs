mod common;

use crate::common::invariants;
use asm_lex::source::lexer::Lexer;
use asm_lex::source::llvm::{targets::*, Llvm};
use asm_lex::source::Item;
use proptest::{collection::vec, prelude::*};

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-x86-elf.txt"))]

    #[test]
    fn x86_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<X86Elf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-x86-darwin.txt"))]

    #[test]
    fn x86_darwin(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<X86Darwin>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-aarch64-elf.txt"))]

    #[test]
    fn aarch64_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<Aarch64Elf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-aarch64-darwin.txt"))]

    #[test]
    fn aarch64_darwin(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<Aarch64Darwin>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-arm-elf.txt"))]

    #[test]
    fn arm_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<ArmElf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-arm-darwin.txt"))]

    #[test]
    fn arm_darwin(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<ArmDarwin>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-riscv-elf.txt"))]

    #[test]
    fn riscv_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<RiscvElf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("llvm-riscv-darwin.txt"))]

    #[test]
    fn riscv_darwin(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Llvm<RiscvDarwin>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf_or_cr(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

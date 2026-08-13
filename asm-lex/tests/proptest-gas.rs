mod common;

use crate::common::invariants;
use asm_lex::source::gas::{targets::*, Gas};
use asm_lex::source::lexer::Lexer;
use asm_lex::source::Item;
use proptest::{collection::vec, prelude::*};

proptest! {
    #![proptest_config(common::proptest::config_file("gas-x86_64-linux-elf.txt"))]

    #[test]
    fn x86_64_linux_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<X86_64LinuxElf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("gas-aarch64-linux-elf.txt"))]

    #[test]
    fn aarch64_linux_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<Aarch64LinuxElf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("gas-arm-linux-eabi-elf.txt"))]

    #[test]
    fn arm_linux_eabi_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<ArmLinuxEabiElf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(common::proptest::config_file("gas-riscv64-linux-elf.txt"))]

    #[test]
    fn riscv64_linux_elf(bytes in vec(common::proptest::asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<Riscv64LinuxElf>>::new(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_lf(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

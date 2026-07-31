#![allow(clippy::incompatible_msrv)]

mod common;

use asm_lex::source::{
    gas::{targets::*, Gas},
    Item,
};
use common::invariants;
use common::*;
use proptest::{
    collection::vec,
    prelude::*,
    test_runner::{Config, FileFailurePersistence},
};

fn config_file(file: &'static str) -> Config {
    Config {
        failure_persistence: Some(Box::new(FileFailurePersistence::Direct(String::leak::<
            'static,
        >(format!(
            "{}/{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "proptest-regressions/invariants",
            file
        ))))),
        ..Config::default()
    }
}

fn asm_byte() -> impl Strategy<Value = u8> {
    prop_oneof![
        20 => Just(b'\n'), 20 => Just(b' '), 10 => Just(b'\t'),
        20 => b':'..=b'@', 30 => b'!'..=b'/', 20 => b'['..=b'`',
        50=>b'a'..=b'z',20=>b'0'..=b'9', 10 => any::<u8>(),
    ]
}

proptest! {
    #![proptest_config(config_file("gas_x86_64_linux_elf.txt"))]

    #[test]
    fn x86_64_linux_elf(bytes in vec(asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<X86_64LinuxElf>>::from_bytes(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_newline(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(config_file("gas_aarch64_linux_elf.txt"))]

    #[test]
    fn aarch64_linux_elf(bytes in vec(asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<Aarch64LinuxElf>>::from_bytes(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_newline(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(config_file("gas_arm_linux_eabi_elf.txt"))]

    #[test]
    fn arm_linux_eabi_elf(bytes in vec(asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<ArmLinuxEabiElf>>::from_bytes(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_newline(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

proptest! {
    #![proptest_config(config_file("gas_riscv64_linux_elf.txt"))]

    #[test]
    fn riscv64_linux_elf(bytes in vec(asm_byte(), 0..1000))
    {
        let items: Vec<Item> = Lexer::<Gas<Riscv64LinuxElf>>::from_bytes(&bytes).collect();
        invariants::monotonic_valid_spans(&bytes, &items);
        invariants::starts_line_iff_newline(&bytes, &items);
        invariants::containing_item_spans(&items);
    }
}

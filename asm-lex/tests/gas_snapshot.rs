mod common;

use asm_lex::source::gas::targets::*;
use asm_lex::source::gas::*;
use common::*;

#[test]
fn x86_64_linux_elf() {
    insta::glob!("fixtures/gas/x86_64_linux_elf/*.s", |path| {
        let bytes = std::fs::read(path).unwrap();
        let lexer = snapshot::SourceLexer::<Gas<X86_64LinuxElf>>::from_bytes(&bytes);
        let snap_items: Vec<snapshot::Item> = lexer.collect();
        insta::assert_debug_snapshot!(snap_items);
    });
}

use proptest::prelude::*;
use proptest::test_runner::{Config, FileFailurePersistence};

#[allow(clippy::incompatible_msrv)]
pub fn config_file(file: &'static str) -> Config {
    Config {
        failure_persistence: Some(Box::new(FileFailurePersistence::Direct(String::leak::<
            'static,
        >(format!(
            "{}/{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "tests/proptest-regressions",
            file
        ))))),
        ..Config::default()
    }
}

pub fn asm_byte() -> impl Strategy<Value = u8> {
    prop_oneof![
        1  => Just(b'\0'),
        10 => Just(b'\t'),
        10 => Just(b'\n'),
        10 => Just(b'\r'),
        30 => Just(b' '),
        30 => b'!'..=b'/',
        20 => b'0'..=b'9',
        20 => b':'..=b'@',
        20 => b'A'..=b'Z',
        10 => b'['..=b'`',
        50 => b'a'..=b'z',
        10 => any::<u8>(),
    ]
}

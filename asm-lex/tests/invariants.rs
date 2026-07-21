mod common;

use asm_lex::source::gas::GasTarget;

fn assert_coverage<T: GasTarget>(bytes: &[u8]) {
    use asm_lex::Span;
    use asm_lex::cursor::Cursor;
    use asm_lex::source::Dialect;
    use asm_lex::source::gas::Gas;

    let mut cursor = Cursor::new(bytes);
    let mut prev_end = 0;
    let mut i = 0;

    while let Some(item) = Gas::<T>::next_item(&mut cursor) {
        let Span { start, end } = item.span();

        assert!(start <= end);
        assert!(end <= bytes.len());
        assert!(prev_end <= start);

        while i < start {
            assert!(
                bytes
                    .get(i)
                    .copied()
                    .is_some_and(|b| T::GAP_CHARS.contains(b))
            );
            i += 1;
        }
        while i < end {
            assert!(
                bytes
                    .get(i)
                    .copied()
                    .is_some_and(|b| !T::GAP_CHARS.contains(b))
            );
            i += 1;
        }

        prev_end = end;
    }

    while i < bytes.len() {
        assert!(
            bytes
                .get(i)
                .copied()
                .is_some_and(|b| T::GAP_CHARS.contains(b))
        );
        i += 1;
    }
}

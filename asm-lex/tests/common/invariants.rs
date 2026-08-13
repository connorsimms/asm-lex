use super::*;
use asm_lex::{source::Item, Span};

// Item spans are
// - monotonically increasing,
// - nonempty (start < end)
// - contained in 0..bytes.len()
pub fn monotonic_valid_spans(bytes: &[u8], items: &[Item]) {
    let bytes_len = bytes.len();
    let mut prev_end = 0usize;
    for item in items {
        let Span { start, end } = item.span();
        assert!(start < end, "Item span {start}..{end} is empty");
        assert!(
            end <= &bytes.len(),
            "Item span {start}..{end} not contained in bytes 0..{bytes_len}",
        );
        assert!(
            *start >= prev_end,
            "Item started at {start} before previous Item's end at {prev_end}",
        );
        prev_end = *end;
    }
}

// TODO: This invariant is ill-formed since LLVM considers '\r' a line-ending
// character. Either create different specializations of this invariant, or
// parameterize the line-separating characters in the trait?
//
// An Item starts a physical line iff it is the first item in the file OR
// there is at least one newline between itself and the previous Item.
pub fn starts_line_iff_newline(bytes: &[u8], items: &[Item]) {
    let mut prev_end = 0usize;
    for item in items {
        let Span { start, end } = item.span();
        if prev_end == 0 {
            assert!(item.starts_line(), "First item must start line");
        } else if item.starts_line() {
            assert!(
                bytes[prev_end..*start].contains(&b'\n'),
                "starts_line = true but no newline found in {:?}",
                prev_end..*start
            );
        } else if !item.starts_line() {
            assert!(
                !bytes[prev_end..*start].contains(&b'\n'),
                "starts_line = false but newline found in {:?}",
                prev_end..*start
            );
        }
        prev_end = *end;
    }
}

// An Item's span must completely contain the subspans of its member `kind`
pub fn containing_item_spans(items: &[Item]) {
    for item in items {
        let span = item.span();
        for_each_span(item.kind(), |field, sub_span| {
            assert!(
                span.start <= sub_span.start && sub_span.end <= span.end,
                "Item span {span:?} does not contain {field:?} span {sub_span:?}"
            );
        });
    }
}

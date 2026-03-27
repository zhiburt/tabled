use quickcheck_macros::quickcheck;

use tabled::settings::width::Wrap;

/// Verify that wrapping preserves all original characters (excluding newlines
/// which are used as line separators by the wrapping algorithm).
///
/// Uses ASCII-only strings to avoid CJK double-width edge cases where a
/// character wider than the wrap width must be replaced.
#[quickcheck]
fn qc_wrap_preserves_all_ascii_characters(bytes: Vec<u8>, width: u8) {
    // Map bytes to printable ASCII + space (0x20..0x7E)
    let text: String = bytes.iter().map(|b| (b % 95 + 32) as char).collect();
    let width = (width as usize).max(1);

    for keep_words in [false, true] {
        let wrapped = Wrap::wrap(&text, width, keep_words);
        let original_chars: Vec<char> = text.chars().filter(|c| *c != '\n').collect();
        let wrapped_chars: Vec<char> = wrapped.chars().filter(|c| *c != '\n').collect();
        assert_eq!(
            original_chars, wrapped_chars,
            "Characters lost or reordered during wrapping (width={width}, keep_words={keep_words})\n\
             original: {text:?}\n\
             wrapped:  {wrapped:?}"
        );
    }
}

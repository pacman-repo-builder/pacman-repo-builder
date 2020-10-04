pub fn split_str_once(text: &str, when: impl Fn(char, usize) -> bool) -> (&str, &str) {
    for (index, current_char) in text.char_indices() {
        if when(current_char, index) {
            return (&text[0..index], &text[index..]);
        }
    }

    (text, "")
}

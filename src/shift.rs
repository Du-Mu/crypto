pub fn shift_char(c: char, key: u8) -> char {
    if c.is_ascii_alphabetic() {
        let base = if c.is_ascii_lowercase() {
            b'a'
        } else {
            b'A'
        };
        let mut value = (c as u8) - base;
        value = (value + key) % 26;
        char::from(base + value)
    } else {
        c
    }
}
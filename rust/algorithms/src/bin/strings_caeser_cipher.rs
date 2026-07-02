fn caesarCipher(s: &str, k: i32) -> String {
    s.bytes()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let mut v = 0;
                if c.is_ascii_lowercase() {
                    v = c - 97;
                } else {
                    v = c - 65;
                }
                return ((v as i32 + k) % 26) as u8 as char;
            } else {
                return c as char;
            }
        })
        .collect()
}

fn caesar_cipher(s: &str, k: i32) -> String {
    s.bytes()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c - base) as i32 + k).rem_euclid(26) + base as i32) as u8 as char
            } else {
                c as char
            }
        })
        .collect()
}

fn main() {}

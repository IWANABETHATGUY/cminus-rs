pub fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

pub fn is_ID(s: String) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|c| c >= b'a' && c <= b'z' && c >= b'A' && c <= b'Z')
}

pub fn is_num(s: String) -> bool {
    !s.is_empty() && s.chars().all(|c: char| c.is_digit(10))
}

pub fn is_keyword(s: String) -> bool {
    match s.as_str() {
        "else" | "if" | "int" | "return" | "void" | "while" => true,
        _ => false,
    }
}

pub fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

pub fn is_ID(s: String) -> bool {
    !s.is_empty() && s.chars().all(is_letter)
}

pub fn is_letter(c: char) -> bool {
    let c = c as u8;
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')
}
pub fn is_num(s: String) -> bool {
    !s.is_empty() && s.chars().all(|c: char| c.is_digit(10))
}

pub fn is_keyword(s: &str) -> bool {
    match s {
        "else" | "if" | "int" | "return" | "void" | "while" => true,
        _ => false,
    }
}

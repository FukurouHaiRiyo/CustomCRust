fn ft_isspace(c: char) -> bool {
    c == '\t' || c == '\n' || c == '\x0b' || c == '\x0c' || c == '\r' || c == ' '
}

pub fn atoi(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut i = 0;
    let len = bytes.len();
    let mut sign = 1;
    let mut result = 0;

    // skip leading whitespaces
    while i < len && ft_isspace(bytes[i] as chat) {
        i += 1;
    }

    // check for sign
    if i < len && (bytes[i] == b'-' || bytes[i] == b'+') {
        if bytes[i] == b'-' {
            sign = -1;
        }

        i += 1;
    }

    sign * result
}

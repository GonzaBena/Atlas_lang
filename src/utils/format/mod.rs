#[allow(dead_code)]
pub fn pad_end(string: &str, length: usize) -> String {
    let mut txt = string.to_string();
    while txt.len() < length {
        txt.push_str(" ");
    }
    txt
}

#[allow(dead_code)]
pub fn pad_start(string: &str, length: usize) -> String {
    let mut txt = String::new();
    while (string.len() + txt.len()) < length {
        txt.push_str(" ");
    }
    txt + string
}

#[allow(dead_code)]
/// Pad a string with spaces at the end to reach a certain length
pub fn pad_end(string: &str, length: usize) -> String {
    let mut txt = string.to_string();
    while txt.len() < length {
        txt.push_str(" ");
    }
    println!("padded: {txt:?}, string: {string}, length: {length}");

    txt
}

#[allow(dead_code)]
/// Pad a string with spaces at the start to reach a certain length
pub fn pad_start(string: &str, length: usize) -> String {
    let mut txt = String::new();
    while (string.len() + txt.len()) < length {
        txt.push_str(" ");
    }
    txt + string
}

#[allow(dead_code)]
/// Center a string in a certain length adding spaces at the start and end
pub fn center(string: &str, length: usize) -> String {
    let mut txt = String::new();
    while txt.len() < length {
        txt.push_str(" ");
    }
    txt.clone() + string + &txt
}

#[cfg(test)]
mod test_format_utils {
    use super::*;

    #[test]
    fn test_pad_end() {
        let string = "hola";
        let padded_string = pad_end(string, 8);
        assert_eq!(padded_string, String::from("hola    "))
    }

    #[test]
    fn test_pad_start() {
        let string = "hola";
        let padded_string = pad_start(string, 8);
        assert_eq!(padded_string, String::from("    hola"))
    }

    #[test]
    fn test_center() {
        let string = "hola";
        let padded_string = center(string, 8);
        assert_eq!(padded_string, String::from("        hola        "))
    }
}

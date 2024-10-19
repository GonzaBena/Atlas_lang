pub struct Color(pub &'static str);

impl Color {
    pub const DEFAULT: &'static str = "\x1B[0m";
    pub const BLACK: &'static str = "\x1B[30m";
    pub const RED: &'static str = "\x1B[31m";
    pub const GREEN: &'static str = "\x1B[32m";
    pub const YELLOW: &'static str = "\x1B[33m";
    pub const BLUE: &'static str = "\x1B[34m";
    pub const MAGENTA: &'static str = "\x1B[35m";
    pub const CYAN: &'static str = "\x1B[36m";
    pub const WHITE: &'static str = "\x1B[37m";

    // Método para colorear un texto
    pub fn colorize(&self, text: &str) -> String {
        format!("{}{}{}", self.0, text, Color::DEFAULT)
    }
}

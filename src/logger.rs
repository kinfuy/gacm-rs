use colorized::{colorize_println, Colors};

pub fn success(text: &str) {
    colorize_println(text, Colors::GreenFg)
}

pub fn error(text: &str) {
    colorize_println(text, Colors::RedFg)
}

pub fn warn(text: &str) {
    colorize_println(text, Colors::YellowFg)
}

pub fn info(text: &str) {
    colorize_println(text, Colors::WhiteFg)
}

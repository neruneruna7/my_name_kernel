use core::fmt;

use alloc::vec::Vec;

use super::{ColorCode, WRITER};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColoredLetter {
    pub ascii_character: char,
    pub color_code: ColorCode,
}

impl ColoredLetter {
    pub fn new(ascii_character: char, color_code: ColorCode) -> Self {
        Self {
            ascii_character,
            color_code,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColoredString(Vec<ColoredLetter>);

impl ColoredString {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from(s: &str, color_code: ColorCode) -> Self {
        let mut colored_string = Self::new();
        for byte in s.chars() {
            colored_string.0.push(ColoredLetter::new(byte, color_code));
        }

        colored_string
    }

    pub fn push(&mut self, s: &str, color_code: ColorCode) {
        let new_colored_string = Self::from(s, color_code);

        for new_s in new_colored_string.0.into_iter() {
            self.0.push(new_s);
        }
    }
}

pub fn color_print(colored_string: ColoredString) {
    // use core::fmt::Write;
    use x86_64::instructions::interrupts;

    // Mutexがロックされているときは割り込みを無効化
    // ただし，一般的な解決策とはならない！
    // レイテンシの時間の最悪値を増加させてしまう
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for s in colored_string.0.into_iter() {
            writer.color_code = s.color_code;
            // writer.write_fmt(args).unwrap();
            writer.write_byte(s.ascii_character as u8);
        }
        // もともとのカラーコードに戻す
        writer.color_code = ColorCode::new(super::Color::Yellow, super::Color::Black);
    });
}

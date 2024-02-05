use futures_util::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

use crate::print;
use crate::vga_buffer::colored_letter::{color_print, ColoredString};
use crate::vga_buffer::{Color, ColorCode};

use super::ScancodeStream;

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => introductions_by_char(character),
                    DecodedKey::RawKey(key) => introductions_by_keycode(key),
                }
            }
        }
    }
}

fn introductions_by_char(char: char) {
    match char {
        'n' => introduction_name(),
        'i' => introduction_icon(),
        _ => print!("{}", char),
    }
}

fn introductions_by_keycode(keycode: KeyCode) {
    match keycode {
        KeyCode::Escape => introduction_icon(),
        _ => print!("{:?}", keycode),
    }
}

pub fn introduction_icon() {
    let icon = r#"
        .-~~~-.
  .- ~ ~-(       )_ _
 /                    ~ -."#;
    let color_code = ColorCode::new(Color::White, Color::Black);
    let colored_string = ColoredString::from(icon, color_code);
    color_print(colored_string);
}

fn introduction_name() {
    let name = "neruneruna7";
    let color_code = ColorCode::new(Color::Black, Color::White);
    let colored_string = ColoredString::from(name, color_code);
    color_print(colored_string);
}

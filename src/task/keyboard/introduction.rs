use alloc::format;
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
        'i' => introduction_icon(),
        'n' => introduction_name(),
        'a' => introduction_age(),
        'l' => introduction_language(),
        'u' => introduction_university(),
        'm' => introduction_major(),
        'g' => introduction_grade(),
        'p' => introduction_programing_language(),
        'q' => introduction_qualification(),
        _ => print!("{}", char),
    }
}

fn introductions_by_keycode(keycode: KeyCode) {
    match keycode {
        KeyCode::Escape => introduction_icon(),
        _ => print!("{:?}", keycode),
    }
}

fn introduction_name() {
    let name = "name: neruneruna7";
    let nick = "neru7";
    let other_name = "peyang gawa ganzisu";
    let sum_name = format!("\n {}\n {}\n {}", name, nick, other_name);
    let color_code = ColorCode::new(Color::Black, Color::White);
    let colored_string = ColoredString::from(&sum_name, color_code);
    color_print(colored_string);
}

fn introduction_age() {
    let s = "\n age: 20";
    let color_code = ColorCode::new(Color::Black, Color::Cyan);
    let colored_string = ColoredString::from(s, color_code);
    color_print(colored_string);
}

fn introduction_language() {
    let s = "\n language: japanese";
    let color_code = ColorCode::new(Color::Red, Color::LightGray);
    let colored_string = ColoredString::from(s, color_code);
    color_print(colored_string);
}

fn introduction_university() {
    let s = "\n university: meijo university";
    let color_code = ColorCode::new(Color::White, Color::Red);
    let colored_string = ColoredString::from(s, color_code);
    color_print(colored_string);
}

fn introduction_major() {
    let s = "\n major: Infomation Engineering";
    let color_code = ColorCode::new(Color::Yellow, Color::Blue);
    let colored_string = ColoredString::from(s, color_code);
    color_print(colored_string);
}

fn introduction_grade() {
    let s = "\n grade: B2";
    let color_code = ColorCode::new(Color::Green, Color::Pink);
    let colored_string = ColoredString::from(s, color_code);
    color_print(colored_string);
}

fn introduction_programing_language() {
    let s = "\n programing language: rust, python, ...";
    let color_code = ColorCode::new(Color::Black, Color::LightCyan);
    let colored_string = ColoredString::from(s, color_code);
    color_print(colored_string);
}

fn introduction_qualification() {
    let s = r"
qualification: 
    AP (Applied Information Technology Engineer Examination)
    nissyo boki syokyuu
    other else";
    let color_code = ColorCode::new(Color::LightGreen, Color::Magenta);
    let colored_string = ColoredString::from(s, color_code);
    color_print(colored_string);
}

pub fn introduction_icon() {
    let bg_color = ColorCode::new(Color::LightGray, Color::DarkGray);
    let or_color = ColorCode::new(Color::Brown, Color::Yellow);
    let he_color = ColorCode::new(Color::Cyan, Color::White);
    let la_color = ColorCode::new(Color::DarkGray, Color::Brown);
    let ye_color = ColorCode::new(Color::Black, Color::Yellow);
    let te_color = ColorCode::new(Color::Yellow, Color::Black);
    let color_str = [
        ("VVyyVyyVyyVyyVfWyyyyyyyyyyZZUUVO111?", bg_color),
        ("+<+11=1zz", or_color),
        ("OVUUZyyyyyyyyyZZZZXZZuuuuzzzzvvvrvv\n", bg_color),
        //
        ("WZZyyWyVVyyyWyyyyyVyyyUVI11", bg_color),
        ("??<><>>>?<>>+>+???====z", or_color),
        ("Ow0XyZZXZZZZZuuZuuuuzzvvrrrrrr\n", bg_color),
        //
        ("ZZZyyyWyyyyyyyyyyyy0C", bg_color),
        ("<<;>>;;;;>>11z+-", or_color),
        ("dOOz??", he_color),
        ("+>>>??>>?+z", or_color),
        ("OXZyZZZZuZZZuuzzzzvvvrrrr\n", bg_color),
        //
        ("ZZyZZZXZZZZZZZZyZX6", bg_color),
        ("?<;;<;;;;;++?1zzzjv611l1++>??>??>>+??=", or_color),
        ("wWZZZZuuXuuzzzzvvvrvrvr\n", bg_color),
        //
        ("ZZZZZZZZZXXZZZyZV", bg_color),
        ("<<;;;::::;;;;+1zz+1+<+<+1<<;;;;>;>>>>+z?=z", or_color),
        ("XXXUuXX0XXzzvvrrrrrrw\n", bg_color),
        //
        ("ZZZZuZZZuZZZZZ0C", bg_color),
        ("<::<:<::::::::::;::<<<<1<<<+::::;;;;>>>>?=lO", or_color),
        ("XZZZZuuuuuzzvrvvrrvr\n", bg_color),
        //
        ("XZyyZyyyZyyyZ0", bg_color),
        ("<<:~~~::~:~:~~::::::<:::::<(<;<::<<<;;;>>??lltr", or_color),
        ("XZZZuuuzzzzzvrrvrrr\n", bg_color),
        //
        ("ZZZZZZZZZZZZ0", bg_color),
        (
            "<<~:~~~~~:~~~~:::::::::~::::;><<<<+;;:<<>+1==ltrw",
            or_color,
        ),
        ("ZuZuuuuuuzuzzzvvzz\n", bg_color),
        //
        ("ZyyyWyyyyyWWC", bg_color),
        (
            "::~_~_~__~~_:~_~:~~~~:::~<:::<::;;:<:<<<>+??=lttwy",
            or_color,
        ),
        ("ZZZuuuuuuuuzzzzzz\n", bg_color),
        //
        ("uuXZZZZZZyyW", bg_color),
        (
            "<:~_~__~~~~.~_~_~~~~:~::_::::_:<;:;:_~~<<<+<1llltv",
            or_color,
        ),
        ("XWyZZZuuuuuuuuuuuu\n", bg_color),
        //
        ("uuuuuuuuZXWW", bg_color),
        (
            "<<:__  _...~..` -.__~_~~::~<:::(<<<:<(_<<<?==llOrv",
            or_color,
        ),
        ("XyyZuXzuuuuuuuXzzz\n", bg_color),
        //
        ("XUZZZZyyVppf2", bg_color),
        (
            "<<__``. ..~_....-_____~~_::::::(<_<:(<<?==llztrvw",
            or_color,
        ),
        ("XWVWXuuXZZZZZuuuww\n", bg_color),
        //
        ("XyXZZyXXWpbpW", bg_color),
        ("+<__`  ...~~~~__~.____~__:::::<:(<;<<=?==llltrvz", or_color),
        ("ZXfffXZyyZyZZZuuuuu\n", bg_color),
        //
        ("yyyWpWfppbbkkk", bg_color),
        ("<:___...._~_~_~~~~_~_~:_~:<;>;;<+?===lllttrrzu", or_color),
        ("XyWpbWWZZyZZZZZuuZuu\n", bg_color),
        //
        ("ZZZZWyVfppbbbbc", bg_color),
        ("<<~~~~_~_~~::<(__::~_((<++<???==1zllltttrwzu", or_color),
        ("ZXpppppfZZZZZZZZZZZuZ\n", bg_color),
        //
        ("uZZZuuXZyVfWWkkz", bg_color),
        ("<;<:_:~<~::<~::_++", or_color),
        ("ewVYT77TTUXAszO", la_color),
        ("zttrwzuXXy", or_color),
        ("WpbppffWyyyZyZZyZZZZZ\n", bg_color),
        //
        ("uuuuuuXZyVffpbbW", bg_color),
        ("z>><<<(;;;<<<+", or_color),
        ("jXY", la_color),
        ("<(-___~~(-(,_?", ye_color),
        ("Wk", la_color),
        ("wrzzuXXy", or_color),
        ("WWkkkbpffyyyyyyZZZZZZZZ\n", bg_color),
        //
        ("ZXXXyyyyyyWpWWkkksz", bg_color),
        ("?>>?+++?+j", or_color),
        ("W=", la_color),
        ("(M@TMfT=~j#BM#US<?", ye_color),
        ("Wk", la_color),
        ("XXXfW", or_color),
        ("   i: icon              \n", te_color),
        //
        ("yyyVyWyyyWVffWpppbkzz", bg_color),
        ("=?z??=jp", or_color),
        ("C~", la_color),
        ("___________________", ye_color),
        ("HW", la_color),
        ("pp", or_color),
        (" n: name, a: age,         \n", te_color),
        //
        ("yZWWWyyyyyWWWWWWWHWbkszz", bg_color),
        ("l=ldW", or_color),
        ("<~", la_color),
        (" koukoiku no sina  ", ye_color),
        ("Wm", la_color),
        ("q", or_color),
        (" l: language, u: university\n", te_color),
        //
        ("WWVXXXyyXyyVWWWWfWbkbpWmAw", bg_color),
        ("OtW", or_color),
        ("n~", la_color),
        ("__________________", ye_color),
        ("jg", la_color),
        ("@", or_color),
        (" m: major, q: qualification ", te_color),
        //
        ("yyyyXXWyyyyZyyWWWWWkkHkkkqq", bg_color),
        ("HmXh", or_color),
        ("JM", la_color),
        ("gM6+g#<MkMd#d@", ye_color),
        ("+j", la_color),
        ("M", or_color),
        (" g: grade, p: programing lang ", te_color),
        //
        ("fffWWWWWfVVVffWyWpWWfWHqqHHkkkqqHma", bg_color),
        ("", or_color),
        ("+++>", la_color),
        (";;", ye_color),
        ("<++jg", la_color),
        ("", or_color),
        ("WHH@@MMgHqkbWWXyZZZyZyyyVVfffppfff", bg_color),
    ];

    for i in color_str.iter() {
        let colored_string = ColoredString::from(i.0, i.1);
        color_print(colored_string);
    }
}

extern crate rand;
use rand::Rng;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn random_string(length: u32, has_number: bool, has_letter: bool,
                     has_uppercase: bool, has_symbol: bool) -> String {
    let mut rng = rand::thread_rng();

    let chars_number = "0123456789";
    let chars_letter = "abcdefghijklmnopqrstuvwxyz";
    let chars_uppercase_letter = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let chars_symbol = "~!@#$%^&*()_+=-[]}{;:,<>?/.";

    let mut all_chars: String = "".into();

    if has_number {
        all_chars += chars_number;
    }
    if has_letter {
        all_chars += chars_letter;
    }
    if has_uppercase {
        all_chars += chars_uppercase_letter;
    }
    if has_symbol {
        all_chars += chars_symbol;
    }

    let mut chars: String = "".into();

    if length < 2 || length > 256 || all_chars.len() < 1 {
        return chars;
    }

    for _ in 0..length {
        let index = rng.gen_range(0..all_chars.len());
        chars += &all_chars[index..index+1];
    }

    chars.to_string()
}

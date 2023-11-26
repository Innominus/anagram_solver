use core::DICTIONARY;
use gloo_worker::oneshot::*;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod core;
pub mod pages;

#[derive(Serialize, Deserialize)]
pub struct HelperStruct {
    reference: String,
    magic_letter: char,
    letter_count: usize,
}

#[oneshot]
pub fn GetFromDictionaryWorker(helper: HelperStruct) -> Vec<String> {
    let mut output_words: Vec<String> = vec![];
    let mut word_does_contain;
    let magic_char_upper = helper.magic_letter.to_uppercase().collect::<Vec<char>>()[0];

    for word in DICTIONARY {
        word_does_contain = true;

        if word.len() < helper.letter_count {
            continue;
        }

        let word_chars: Vec<char> = word.chars().collect();
        let mut reference_chars: Vec<char> = helper.reference.to_uppercase().chars().collect();

        if !word_chars.contains(&magic_char_upper) {
            continue;
        }

        reference_chars.push(magic_char_upper);

        for letter in word_chars {
            if !reference_chars.contains(&letter) {
                word_does_contain = false;
                break;
            }
        }

        if word_does_contain {
            output_words.push(word.to_string());
        }
    }

    output_words
}

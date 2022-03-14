use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::io::{stdout, Write};

static NUMBER_WORDS: usize = 5;

fn main() -> std::io::Result<()> {
    let first_arg = env::args().nth(1).unwrap_or_else(|| "0".to_owned());
    let tmp_first_arg = first_arg
        .parse::<usize>()
        .expect("Please provide valid number");
    let mut number_words = if tmp_first_arg > 0 {
        tmp_first_arg
    } else {
        NUMBER_WORDS
    };

    // or use `include_str!` to embed this dict into the binary
    let words_strings = fs::read_to_string("/usr/share/dict/words")?;
    let mut filter_words: Vec<_> = words_strings
        .par_lines()
        // `matches!(c, 'a'..='z')` is https://doc.rust-lang.org/core/primitive.char.html#method.is_ascii_lowercase
        .filter(|line| line.len() == 5 && line.chars().all(|c| matches!(c, 'a'..='z')))
        .collect();

    if number_words > filter_words.len() {
        number_words = filter_words.len();
    }

    filter_words.shuffle(&mut thread_rng());

    let stdout = stdout();
    let mut lock = stdout.lock();
    for i in 0..number_words {
        writeln!(lock, "{}", &filter_words[i])?;
    }

    Ok(())
}

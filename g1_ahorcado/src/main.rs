use std::io;
use std::str;

fn main() {

    const SECRET_WORD: &str = "manzana";

    let guessed_letters: Vec<char> = Vec::new();

    print_guessed_word(SECRET_WORD, guessed_letters);

}

fn print_guessed_word(secret_word: &str, guessed_letters: Vec<char>) {
    for l in secret_word.chars() {
        if !guessed_letters.contains(l) { print!("_") }
        else { print!("{l}") };
    };
}
// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::collections::HashSet;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn print_word_so_far(word_so_far: &Vec<char>) {
    print!("The word so far is ");
    for c in word_so_far {
        print!("{}", c);
    }
    println!();
}

fn print_word_guessed(word_guessed: &HashSet<char>) {
    print!("You have guessed the following letters: ");
    for c in word_guessed {
        print!("{}", c);
    }
    println!();
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut word_so_far: Vec<char> = Vec::new();
    let mut word_not_guessed = HashSet::new();
    let mut word_guessed = HashSet::new();
    let mut times_remained = NUM_INCORRECT_GUESSES;
    for i in 0..secret_word.len() {
        word_so_far.push('-');
        word_not_guessed.insert(secret_word_chars[i]);
    }

    loop {
        print_word_so_far(&word_so_far);
        print_word_guessed(&word_guessed);
        println!("You have {} guesses left", times_remained);
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guessed_char: Vec<char> = guess.chars().collect();
        if word_not_guessed.contains(&guessed_char[0]) {
            word_not_guessed.remove(&guessed_char[0]);
            word_guessed.insert(guessed_char[0]);
            for i in 0..secret_word_chars.len() {
                if secret_word_chars[i] == guessed_char[0] {
                    word_so_far[i] = guessed_char[0];
                }
            }
        } else {
            times_remained -= 1;
            word_guessed.insert(guessed_char[0]);
            println!("Sorry, that letter is not in the word");
        }
        println!();
        if times_remained == 0 {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        if word_not_guessed.is_empty() {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }
    }
}

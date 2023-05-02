use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use unicode_segmentation::UnicodeSegmentation;

const WORD_LEN: usize = 5;

fn main() {
    let word = get_random_word("words.txt");
    let mut round = 1;

    while round <= 6 {
        println!("Round {}: Type your guess", round);
        let input = get_user_input();

        if !is_valid_input(&input) {
            continue;
        }

        match calc_round(&word, &input) {
            GuessResult::Correct => {
                println!("You win!");
                return;
            }
            GuessResult::Incorrect(hint) => {
                println!("\nNot quite!\n");
                println!("{}\n{}\n", hint, input);
            }
        }

        round += 1;
    }

    println!("Ahh shucks, good luck next time!");
}

const G: char = 'G';
const Y: char = 'Y';
const X: char = 'X';

#[derive(PartialEq, Debug)]
enum GuessResult {
    Correct,
    Incorrect(String),
}

fn calc_round(word: &str, input: &str) -> GuessResult {
    let mut correct_positions = 0;

    let mut hint = String::new();
    for (i, c) in input.graphemes(true).enumerate() {
        if word.graphemes(true).nth(i).unwrap() == c {
            hint.push(G);
            correct_positions += 1;
            continue;
        }
        if word.contains(c) {
            hint.push(Y);
        } else {
            hint.push(X);
        }
    }

    match correct_positions {
        WORD_LEN => GuessResult::Correct,
        _ => GuessResult::Incorrect(hint),
    }
}

fn is_valid_input(input: &str) -> bool {
    match input.trim().graphemes(true).count() {
        WORD_LEN => true,
        _ => {
            println!("\nPlease enter a {} letter word\n", WORD_LEN);
            false
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read user input");

    input.trim().to_string()
}

fn get_random_word(filepath: &str) -> String {
    let file = File::open(&filepath).unwrap();
    let reader = BufReader::new(file);

    let num_words = reader.lines().count();
    let rnd_index = rand::thread_rng().gen_range(0..num_words);

    let file = File::open(&filepath).unwrap();
    let reader = BufReader::new(file);

    let random_word = reader
        .lines()
        .nth(rnd_index)
        .expect("Could not get random word")
        .unwrap();

    random_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random_word_test() {
        let res = get_random_word("words_test.txt");
        assert_eq!(res, "hello".to_string());
    }

    #[test]
    fn valid_word() {
        let res = is_valid_input("hello");
        assert_eq!(res, true);
    }

    #[test]
    fn invalid_word() {
        let res = is_valid_input("helloooooo");
        assert_eq!(res, false);
    }

    #[test]
    fn correct_guess() {
        let res = calc_round("hello", "hello");
        assert_eq!(res, GuessResult::Correct);
    }

    #[test]
    fn incorrect_guess() {
        let res = calc_round("hello", "world");
        assert_eq!(res, GuessResult::Incorrect("XYXGX".to_string()));
    }
}

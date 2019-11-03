use crate::hangman::Hangman;
use crate::utils::{get_available_letters, is_word_guessed, show_possible_words};
use std::io::stdin;
use std::process;

pub mod hangman;
pub mod utils;
pub mod words;

pub fn display_info(hangman: &Hangman) {
  if hangman.get_warnings() < 3 {
    println!("You have {} warnings left", hangman.get_warnings());
  }
  println!("You have {} guesses left.", hangman.get_guesses());
  println!(
    "Available letters: {}",
    get_available_letters(&hangman.get_letters_guessed())
  );
}

pub fn is_game_active(hangman: &Hangman) -> bool {
  hangman.get_guesses() > 0
    && !is_word_guessed(&hangman.get_secret_word(), &hangman.get_letters_guessed())
}

pub fn print_win(hangman: &Hangman) {
  println!("Congratulations, you win!");
  println!(
    "Your total score for this game is: {}",
    hangman.get_total_score()
  );
  process::exit(0);
}

pub fn user_guess(hangman: &mut Hangman) {
  println!("Please guess a letter: ");
  let mut input = String::new();
  stdin()
    .read_line(&mut input)
    .expect("Failed to react guess");
  let guess: char = input.bytes().nth(0).expect("No input detected") as char;
  match guess {
    '*' => {
      let possible_words =
        show_possible_words(&hangman.get_words_guessed(), hangman.get_word_list());
      if let Some(possible_words) = possible_words {
        println!("{}", possible_words);
      } else {
        println!("Oops! No matched words");
      }
    }
    _ => {
      match guess.is_ascii_alphabetic() {
        false => {
          hangman.decrement_warning();
          println!(
            "Oops! That is not a valid letter. You have {} warnings left: {}",
            hangman.get_warnings(),
            hangman.get_words_guessed()
          )
        }
        true => {
          determine_has_guessed(hangman, &guess);
        }
      };
    }
  }
  println!("____________");
}

fn determine_has_guessed(hangman: &mut Hangman, guess: &char) {
  match hangman.is_guess_in_secret(guess) {
    false => {
      let vowels = vec!['a', 'e', 'i', 'o', 'u'];
      if vowels.contains(guess) {
        hangman.decrement_guess_vowel();
      } else {
        hangman.decrement_guesses();
      }
      println!(
        "Oops!, That is not in my word: {}",
        hangman.get_words_guessed()
      );
    }
    true => {
      if hangman.get_letters_guessed().iter().any(|e| e == guess) {
        hangman.decrement_warning();
        println!(
          "Oops!, You've already guessed that letter. You have {} warnings left",
          hangman.get_warnings()
        );
      } else {
        // the user has not guessed the char
        hangman.add_letters_guessed(*guess);
        println!("Good guess: {}", hangman.get_words_guessed());
      }
    }
  }
}

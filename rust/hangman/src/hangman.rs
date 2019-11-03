use super::utils::{exit_game, get_guessed_word};
use super::words::HangmanWords;
use std::collections::HashSet;

pub struct Hangman {
  secret_word: String,
  guesses: usize,
  word_list: HangmanWords,
  letters_guessed: Vec<char>,
  warnings: usize,
}

impl Hangman {
  pub fn new() -> Hangman {
    let word_list = HangmanWords::new("words.txt");
    let secret_word = word_list.choose_word(); //curl
    Hangman {
      secret_word,
      word_list,
      guesses: 6,
      warnings: 3,
      letters_guessed: Vec::new(),
    }
  }

  pub fn get_warnings(&self) -> usize {
    self.warnings
  }

  pub fn get_guesses(&self) -> usize {
    self.guesses
  }

  pub fn get_word_list(&self) -> &Vec<String> {
    self.word_list.get_words()
  }

  pub fn get_letters_guessed(&self) -> &Vec<char> {
    &self.letters_guessed
  }

  pub fn get_secret_word(&self) -> &String {
    &self.secret_word
  }

  pub fn add_letters_guessed(&mut self, guess: char) {
    self.letters_guessed.push(guess);
  }

  pub fn guess_len(&self) -> usize {
    self.secret_word.len()
  }

  pub fn is_guess_in_secret(&mut self, guess: &char) -> bool {
    let guess_lower = guess.to_lowercase().collect::<Vec<char>>();
    self.secret_word.contains(guess_lower[0])
  }

  pub fn get_words_guessed(&self) -> String {
    get_guessed_word(&self.secret_word, &self.letters_guessed)
  }

  pub fn decrement_warning(&mut self) {
    if self.warnings > 0 {
      self.warnings -= 1;
    } else if self.warnings == 0 {
      self.decrement_guesses();
    }
  }

  pub fn decrement_guess_vowel(&mut self) {
    if self.guesses == 2 {
      exit_game(&self.secret_word);
    } else {
      self.guesses -= 2;
    }
  }

  pub fn decrement_guesses(&mut self) {
    if self.guesses == 1 {
      exit_game(&self.secret_word);
    } else {
      self.guesses -= 1;
    }
  }

  fn get_unique_letters(&self) -> usize {
    let mut secret_chars = self.secret_word.chars().collect::<Vec<char>>();
    let mut uniques = HashSet::new();
    secret_chars.retain(|a| uniques.insert(a.clone()));
    secret_chars.len()
  }

  pub fn get_total_score(&self) -> usize {
    self.guesses * self.get_unique_letters()
  }
}

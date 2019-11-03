use rand::Rng;
use std::fs::File;
use std::io::Read;

// Contains a list of valid words. Words are strings of lowercase letters.
// Depending on the size of the word list, this function may
// take a while to finish.
pub struct HangmanWords {
  words: Vec<String>,
}

impl HangmanWords {
  pub fn new(filename: &str) -> HangmanWords {
    println!("Loading word list from file...");

    let mut f = File::open(filename).expect("Error loading file");
    let mut words: Vec<String> = Vec::new();
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Error parsing file");
    // convert word to a vector of words
    for i in s.split(" ") {
      words.push(i.to_string());
    }
    println!("{} words loaded", words.len());
    HangmanWords { words }
  }

  pub fn choose_word(&self) -> String {
    let random_index = rand::thread_rng().gen_range(1, word_list.len() + 1);
    // let random_index = 1000;
    // TODO: Refactor to use box
    self.words[random_index].clone()
  }

  pub fn get_words(&self) -> &Vec<String> {
    &self.words
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_loads_word() {
    let words = HangmanWords::new("words.txt");
    assert_eq!(words.words.len(), 55900);
  }
  #[test]
  #[should_panic]
  fn it_should_panic() {
    HangmanWords::new("wrong-words.txt");
  }
}

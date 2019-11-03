use std::process;

pub fn is_word_guessed(secret_word: &String, letters_guessed: &Vec<char>) -> bool {
  secret_word.chars().all(|x| letters_guessed.contains(&x))
}

pub fn get_guessed_word(secret_word: &String, letters_guessed: &Vec<char>) -> String {
  let mut word: Vec<String> = vec!["_ ".to_string(); secret_word.len()];

  for (index, letter) in secret_word.chars().enumerate() {
    if letters_guessed.contains(&letter) {
      word[index] = letter.to_string();
    }
  }
  return word.concat();
}

pub fn get_available_letters(letters_guessed: &Vec<char>) -> String {
  let alphabets = String::from("abcdefghijklmnopqrstuvwxyz");

  let filtered = alphabets
    .chars()
    .filter(|c| !letters_guessed.contains(&c))
    .collect::<String>();

  return filtered;
}

pub fn exit_game(secret_word: &String) {
  println!("Oops! you lost the game, you ran out of guesses");
  println!("My secret guess is: {}", secret_word);
  process::exit(0);
}

pub fn match_with_gaps(word_a: &String, word_b: &String) -> bool {
  let word_a_strip = word_a.trim().replace(" ", "");
  let word_b_strip = word_b.trim().replace(" ", "");

  if word_a_strip.len() != word_b_strip.len() {
    return false;
  }
  for (i, c) in word_a_strip.char_indices() {
    if c != '_' {
      if word_a_strip.matches(c).collect::<Vec<_>>().len()
        != word_b_strip.matches(c).collect::<Vec<_>>().len()
      {
        return false;
      }

      if let Some(n) = word_b_strip.chars().nth(i) {
        if n != c {
          return false;
        }
      }
    }
  }

  true
}

pub fn show_possible_words(my_word: &String, words: &Vec<String>) -> Option<String> {
  let mut matched_words: Vec<String> = Vec::new();
  for (_, word) in words.iter().enumerate() {
    if match_with_gaps(&my_word, &word) {
      matched_words.push(word.clone());
    }
  }

  if matched_words.len() > 0 {
    let word_string = matched_words.join(" ");
    return Some(word_string);
  }
  None
}

#[cfg(test)]
mod is_word_guessed_tests {
  use super::is_word_guessed;

  #[test]
  fn word_guessed() {
    let guessed = is_word_guessed(&"apple".to_string(), &vec!['e', 'p', 'l', 'p', 'a', 's']);
    assert!(guessed);
  }

  #[test]
  fn word_not_guessed() {
    let guessed = is_word_guessed(&"apple".to_string(), &vec!['e', 'i', 'k', 'r', 's']);
    assert!(!guessed);
  }
}

#[cfg(test)]
mod get_guessed_word_tests {
  use super::get_guessed_word;

  #[test]
  fn gets_guessed_parts() {
    let guessed = get_guessed_word(&"apple".to_string(), &vec!['e', 'i', 'k', 'p', 'r', 's']);
    assert_eq!(guessed, "_ pp_ e");
  }

  #[test]
  fn gets_all_parts() {
    let guessed = get_guessed_word(&"apple".to_string(), &vec!['a', 'l', 'k', 'p', 'e', 's']);
    assert_eq!(guessed, "apple");
  }

  #[test]
  fn gets_no_parts() {
    let guessed = get_guessed_word(&"apple".to_string(), &vec!['s', 'd', 'f', 'j', 'g', 'm']);
    assert_eq!(guessed, "_ _ _ _ _ ");
  }
}

#[cfg(test)]
mod test_get_available_letters {
  use super::get_available_letters;

  #[test]
  fn removed_guessed_letters() {
    let letters = get_available_letters(&vec!['e', 'p', 'l', 'p', 'a', 's']);
    assert_eq!(letters, "bcdfghijkmnoqrtuvwxyz");
  }

  #[test]
  fn remove_empty_guessed_letters() {
    let letters = get_available_letters(&vec![]);
    assert_eq!(letters, "abcdefghijklmnopqrstuvwxyz");
  }
}

#[cfg(test)]
mod match_with_gaps_test {
  use super::match_with_gaps;

  #[test]
  fn not_matched_1() {
    assert!(!match_with_gaps(&"te_ t".to_string(), &"tact".to_string()));
  }

  #[test]
  fn not_matched_with_len() {
    assert!(!match_with_gaps(
      &"a_ _ le".to_string(),
      &"banana".to_string()
    ));
  }

  #[test]
  fn matched_1() {
    assert!(match_with_gaps(
      &"a_ _ le".to_string(),
      &"apple".to_string()
    ));
  }

  #[test]
  fn not_matched_with_close_finished() {
    assert!(!match_with_gaps(
      &"a_ ple".to_string(),
      &"apple".to_string()
    ));
  }

  #[test]
  fn not_matched_with_char() {
    assert!(!match_with_gaps(&"ta_ _ ".to_string(), &"tact".to_string()));
  }

  #[test]
  fn matched_4() {
    assert!(match_with_gaps(&"t_ _ t".to_string(), &"tact".to_string()));
  }
}

#[cfg(test)]
mod show_possible_words_test {
  use super::show_possible_words;
  #[test]
  fn it_gets_possible_matches() {
    let words: Vec<String> = vec![
      String::from("apple"),
      String::from("tact"),
      String::from("tart"),
      String::from("taut"),
      String::from("trent"),
      String::from("twit"),
    ];

    let my_word = String::from("t_ _ t");

    assert_eq!(
      show_possible_words(&my_word, &words),
      Some(String::from("tact tart taut twit"))
    );
  }
}

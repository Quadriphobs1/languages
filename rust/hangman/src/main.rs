use hangman::hangman::Hangman;
use hangman::{display_info, is_game_active, print_win, user_guess};

fn main() {
  let mut game = Hangman::new();
  println!("Welcome to the game Hangman!");
  println!(
    "I am thinking of a word that is {} letters long. \n _______________",
    game.guess_len()
  );
  while is_game_active(&game) {
    display_info(&game);
    user_guess(&mut game);
  }

  print_win(&game);
}

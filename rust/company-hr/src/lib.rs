pub mod action;
pub mod department;
pub mod employee;

pub fn title_case(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f
      .to_uppercase()
      .chain(c.flat_map(|t| t.to_lowercase()))
      .collect(),
  }
}

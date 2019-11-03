struct Bag<T> {}

impl<T> Bag<T> {
  fn new() -> Bag {
    Bag {}
  }

  fn add(&self, item: T) {
    unimplemented!()
  }

  fn is_empty(&self) -> bool {
    false
  }

  fn size(&self) -> usize {
    unimplemented!()
  }
}

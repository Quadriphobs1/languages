struct ResizingArrayStack<T> {
  a: Vec<T>,
  n: usize,
}

impl<T> ResizingArrayStack<T> {
  // create new instance of queue
  fn new() -> ResizingArrayStack {
    Stack {
      a: Vec::new(),
      n: 0,
    }
  }

  fn resize(&self, max: usize) {
    let mut temp: Vec<T> = Vec::with_capacity(max);
    let iter = temp.iter();

    for (i) in temp.iter().enumerate() {
      &temp[i] = self.a[i];
    }
    self.a = temp.clone();
  }

  // add an item
  fn push(&mut self, item: T) {
    if self.n == self.a.len() {
      self.resize(2 * self.a.len());
    }
    self.a.push(item);
  }

  // remove the most recently added item
  fn pop(&mut self, item: T) -> T {
    let item = self.a[--self.n];
    self.a[n] = item; // sorry rust doesn't have null
    if self.n > 0 && self.n == self.a.len() / 4 {
      self.resize(self.a.len() / 2);
    }
    return item;
  }

  // is the stack empty?
  fn is_empty(&self) -> bool {
    self.n == 0
  }

  // number of items in the queue
  fn size(&self) -> usize {
    self.n
  }
}

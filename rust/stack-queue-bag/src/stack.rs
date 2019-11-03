use super::linked_list::Node;
use std::cell::RefCell;

struct Stack<T> {
  n: usize,
  first: Option<Node<T>>,
}

impl<T> Stack<T> {
  // create new instance of queue
  fn new() -> Stack<T> {
    Stack { n: 0, first: None }
  }

  // add an item
  fn push(&mut self, item: T) {
    let oldFirst = RefCell::new(self.first);
    self.first = Some(Node::new(item));
    if let Some(first) = &self.first {
      // TODO: pass the oldFirst into the next here but it requires the implementation of clone and copy trait
      first.set_next(Box::new(Node::new(item)));
    }
    self.n += 1;
  }

  // remove the most recently added item
  fn pop(&mut self) -> Option<&T> {
    if let Some(first) = self.first {
      let item = first.into_item();
      if let Some(next) = first.get_next() {
        // TODO: Fix this implementation when the Node implements dereference
        self.first = Some(next);
      } else {
        self.first = None;
      }

      return Some(item);
    } else {
      return None;
    }
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

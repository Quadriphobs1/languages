use core::ptr::NonNull;

pub struct Node<T> {
  next: Option<NonNull<Node<T>>>,
  item: T,
}

impl<T> Node<T> {
  pub fn new(item: T) -> Node<T> {
    Node { item, next: None }
  }

  pub fn set_next(&mut self, next: Box<Node<T>>) {
    let next_ptr = Some(Box::into_raw_non_null(next));
    self.next = next_ptr;
  }

  pub fn next_into_item(&self) -> Option<&T> {
    unsafe { self.next.as_ref().map(|node| &node.as_ref().item) }
  }

  pub fn get_next(&self) -> Option<&Node<T>> {
    unsafe { self.next.as_ref().map(|node| node.as_ref()) }
  }

  pub fn into_item(&self) -> &T {
    &self.item
  }
}

// impl<T> Copy for Node<T> {}

// impl<T> Clone for Node<T> {
//   fn clone(&self) -> Self {
//     *self
//   }
// }

#[cfg(test)]
mod test_node {
  use super::*;
  #[test]
  fn it_has_item() {
    let node: Node<i32> = Node::new(1);
    assert_eq!(node.into_item(), &1);
    assert_eq!(node.next_into_item(), None);
  }
  #[test]
  fn it_adds_next() {
    let mut node: Node<i32> = Node::new(1);
    let next: Node<i32> = Node::new(2);
    node.set_next(Box::new(next));
    assert_eq!(node.next_into_item(), Some(&2));
  }
}

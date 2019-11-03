struct Queue<T> {}

impl<T> Queue<T> {
  // create new instance of queue
  fn new() -> Queue {
    Queue {}
  }

  // add an item
  fn enqueue(&self, item: T) {
    unimplemented!()
  }

  // remove the last recently added item
  fn dequeue(&self, item: T) -> T {
    unimplemented!()
  }

  // is the queue empty?
  fn is_empty(&self) -> bool {
    false
  }

  // number of items in the queue
  fn size(&self) -> usize {
    unimplemented!()
  }
}

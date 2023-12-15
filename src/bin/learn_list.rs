use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Rev;
use std::marker::PhantomData;
use std::ops::Deref;

/// List's node
struct Node<T> {
  data: Option<T>,
  last: Option<Rc<RefCell<Node<T>>>>,
  next: Option<Rc<RefCell<Node<T>>>>,
}

type NodePtr<T> = Rc<RefCell<Node<T>>>;
pub type NullableNodePtr<T> = Option<NodePtr<T>>;

/// A normal double linked list
pub struct List<T> {
  len: usize,
  head: NullableNodePtr<T>,
  tail: NullableNodePtr<T>,
}

impl<T> Node<T> {
  fn new_ptr(data: T) -> NodePtr<T> {
    Rc::new(RefCell::new(Node {
      data: Some(data),
      last: None,
      next: None,
    }))
  }
}

impl<T> List<T> {
  /// Create an empty list
  pub fn new() -> Self {
    List {
      len: 0,
      head: None,
      tail: None,
    }
  }

  /// Get size of the list
  pub fn len(&self) -> usize {
    self.len
  }

  /// If list is empty
  pub fn empty(&self) -> bool {
    self.len == 0
  }

  /// Link a new node to list's tail
  pub fn push_back(&mut self, data: T) {
    let new_node = Node::new_ptr(data);

    match self.tail.take() {
      None => {
        self.head = Some(new_node.clone());
      }
      Some(old_tail) => {
        old_tail.borrow_mut().next = Some(new_node.clone());
        new_node.borrow_mut().last = Some(old_tail);
      }
    }

    self.tail = Some(new_node);
    self.len += 1;
  }

  /// Link a new node to list's head
  pub fn push_front(&mut self, data: T) {
    let new_node = Node::new_ptr(data);

    match self.head.take() {
      None => {
        self.tail = Some(new_node.clone());
      }
      Some(old_head) => {
        old_head.borrow_mut().last = Some(new_node.clone());
        new_node.borrow_mut().next = Some(old_head);
      }
    }

    self.head = Some(new_node);
    self.len += 1;
  }

  /// Remove a node from the tail
  pub fn pop_back(&mut self) -> Option<T> {
    match self.tail.take() {
      None => { None }
      Some(old_tail) => {
        self.tail = old_tail.borrow_mut().last.take();
        self.len -= 1;

        match self.tail.as_ref() {
          None => {
            self.head = None;
          }
          Some(&ref new_tail) => {
            new_tail.borrow_mut().next = None;
          }
        }

        old_tail.borrow_mut().data.take()
      }
    }
  }

  /// Remove a node from the head
  pub fn pop_front(&mut self) -> Option<T> {
    match self.head.take() {
      None => { None }
      Some(old_head) => {
        self.head = old_head.borrow_mut().next.take();
        self.len -= 1;

        match self.head.as_ref() {
          None => {
            self.tail = None;
          }
          Some(&ref new_head) => {
            new_head.borrow_mut().last = None;
          }
        }

        old_head.borrow_mut().data.take()
      }
    }
  }
}


/// Iterator to iterate list
pub struct Iter<'a, T: 'a> {
  curr: &'a NullableNodePtr<T>,
}

/// Mutable iterator to iterate list
pub struct MutIter<'a, T: 'a> {
  curr: &'a mut NullableNodePtr<T>,
}

impl<T> List<T> {
  /// Create an iterator begin from list's head
  pub fn iter(&self) -> Iter<'_, T> {
    Iter { curr: &self.head }
  }

  pub fn mut_iter(&mut self) -> MutIter<'_, T> {
    MutIter { curr: &mut self.head }
  }

  pub fn rev_iter(&self) -> Rev<Iter<'_, T>> {
    Iter { curr: &self.tail }.rev()
  }

  pub fn mut_rev_iter(&mut self) -> Rev<MutIter<'_, T>> {
    MutIter { curr: &mut self.tail }.rev()
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.curr.as_ref() {
      None => { None }
      Some(curr) => unsafe {
        self.curr = &(*curr.as_ptr()).next;
        (*curr.as_ptr()).data.as_ref()
      }
    }
  }
}

impl<'a, T> Iterator for MutIter<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.curr.as_ref() {
      None => { None }
      Some(curr) => unsafe {
        self.curr = &mut (*curr.as_ptr()).next;
        (*curr.as_ptr()).data.as_mut()
      }
    }
  }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    match self.curr.as_ref() {
      None => {None}
      Some(curr) => unsafe {
        self.curr = &(*curr.as_ptr()).last;
        (*curr.as_ptr()).data.as_ref()
      }
    }
  }
}

impl<'a, T> DoubleEndedIterator for MutIter<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    match self.curr.as_ref() {
      None => {None }
      Some(curr) => unsafe {
        self.curr = &mut(*curr.as_ptr()).last;
        (*curr.as_ptr()).data.as_mut()
      }
    }
  }
}

fn main() {
  struct MyData {
    x: String,
  }

  let mut ls = List::new();

  ls.push_front(MyData { x: String::from("1") });
  ls.push_back(MyData { x: String::from("2") });
  ls.push_front(MyData { x: String::from("3") });
  ls.push_back(MyData { x: String::from("4") });

  for i in ls.iter() {
    println!("{}", i.x)
  }

  for i in ls.mut_iter() {
    i.x.push('!');
  }

  for i in ls.rev_iter() {
    println!("- {}", i.x);
  }

  while !ls.empty() {
    match ls.pop_back() {
      None => {
        println!("none");
      }
      Some(_x) => {
        println!("{}", _x.x);
      }
    }
  }
}

use std::cell::RefCell;
use std::collections::LinkedList;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let x = Box::new(1);
    let y = x.deref();


    let a = Rc::new(2);
    let b = a.deref();

    let n = RefCell::new(3);
    let m = n.borrow().deref();

    let mut ls = LinkedList::new();
    ls.push_front(1);

    let mut x = Vec::new();
    x.push(1);
    x.iter().rev();
}

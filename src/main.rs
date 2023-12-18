use std::cell::RefCell;
use std::collections::LinkedList;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    for i in (-1..3).rev() {
        println!("{i}");
    }
}

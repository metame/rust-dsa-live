#![allow(dead_code)]

use crate::array_list::ArrayList;
use crate::linked_list::LinkedList;

/* implementations based on crate's collections */

pub struct ArrayStack<T> {
    list: ArrayList<T>
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        ArrayStack { list: ArrayList::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
}


pub struct LinkedStack<T> {
    list: LinkedList<T>
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        LinkedStack { list: LinkedList::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push_front(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

/* implementations based on stdlib collections */

pub struct StdArrayStack<T> {
    list: Vec<T>
}

impl<T> StdArrayStack<T> {
    pub fn new() -> Self {
        StdArrayStack { list: Vec::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
}

pub struct StdLinkedStack<T> {
    list: std::collections::LinkedList<T>
}

impl<T> StdLinkedStack<T> {
    pub fn new() -> Self {
        StdLinkedStack { list: std::collections::LinkedList::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push_front(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

mod tests {
    use super::*;

    #[test]
    fn array_stack_works() {
        let mut s = ArrayStack::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(3478));
    }

    #[test]
    fn linked_stack_works() {
        let mut s = LinkedStack::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(3478));
    }

    #[test]
    fn std_array_stack_works() {
        let mut s = StdArrayStack::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(3478));
    }

    #[test]
    fn std_linked_stack_works() {
        let mut s = StdLinkedStack::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(3478));
    }
}

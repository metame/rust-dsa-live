#![allow(dead_code)]

use std::collections::{LinkedList, VecDeque};

use crate::array_list::ArrayList;
use crate::doubly_linked_list::DoublyLinkedList;

/* implementations based on crate's collections */

pub struct ArrayQueue<T> {
    list: ArrayList<T>
}

impl<T> ArrayQueue<T> {
    pub fn new() -> Self {
        ArrayQueue { list: ArrayList::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push_front(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
}


pub struct LinkedQueue<T> {
    list: DoublyLinkedList<T>
}

impl<T> LinkedQueue<T> {
    pub fn new() -> Self {
        LinkedQueue { list: DoublyLinkedList::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push_front(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

/* implementations based on stdlib collections */

pub struct StdArrayQueue<T> {
    list: VecDeque<T>
}

impl<T> StdArrayQueue<T> {
    pub fn new() -> Self {
        StdArrayQueue { list: VecDeque::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push_front(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

pub struct StdLinkedQueue<T> {
    list: LinkedList<T>
}

impl<T> StdLinkedQueue<T> {
    pub fn new() -> Self {
        StdLinkedQueue { list: LinkedList::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.list.push_front(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

mod tests {
    use super::*;

    #[test]
    fn array_queue_works() {
        let mut s = ArrayQueue::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(3478));
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn linked_queue_works() {
        let mut s = LinkedQueue::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(3478));
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn std_array_queue_works() {
        let mut s = StdArrayQueue::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(3478));
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn std_linked_queue_works() {
        let mut s = StdLinkedQueue::<usize>::new();
        assert_eq!(s.pop(), None);
        s.push(3478);
        s.push(1);
        assert_eq!(s.pop(), Some(3478));
        assert_eq!(s.pop(), Some(1));
    }
}

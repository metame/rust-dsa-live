#![allow(dead_code)]
use std::sync::{Arc, Mutex};

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    len: usize,
    first: Link<T>,
    last: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            len: 0,
            first: None,
            last: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let mut n = Node::new(value);
        if let Some(old_first) = self.first.take() {
            n.next = Some(old_first.clone());
            let link = Arc::new(Mutex::new(n));
            self.first = Some(link.clone());
            old_first
                .clone()
                .lock()
                .map(|mut n| {
                    n.prev = Some(link);
                })
                .unwrap();
        } else {
            let link = Arc::new(Mutex::new(n));
            self.first = Some(link.clone());
            self.last = Some(link);
        }
        self.len += 1;
    }

    fn push_back(&mut self, value: T) {
        let mut n = Node::new(value);
        if let Some(old_last) = self.last.take() {
            n.prev = Some(old_last.clone());
            let link = Arc::new(Mutex::new(n));
            self.last = Some(link.clone());
            old_last
                .clone()
                .lock()
                .map(|mut n| {
                    n.next = Some(link);
                })
                .unwrap();
        } else {
            let link = Arc::new(Mutex::new(n));
            self.first = Some(link.clone());
            self.last = Some(link);
        }
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        if let Some(first) = self.first.take() {
            first
                .lock()
                .map(|mut old_first| {
                    if let Some(first_node) = old_first.next.take() {
                        self.first = Some(first_node.clone());
                        let mut first_node = first_node.lock().unwrap();
                        first_node.prev = None;
                    } else {
                        self.last.take();
                    }
                })
                .unwrap();
            self.len -= 1;

            Arc::into_inner(first)
                .and_then(|m| m.into_inner().ok())
                .map(|n| n.value)
        } else {
            None
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        if let Some(last) = self.last.take() {
            last
                .lock()
                .map(|mut old_last| {
                    if let Some(last_node) = old_last.prev.take() {
                        self.last = Some(last_node.clone());
                        let mut last_node = last_node.lock().unwrap();
                        last_node.next = None;
                    } else {
                        self.first.take();
                    }
                })
                .unwrap();
            self.len -= 1;

            Arc::into_inner(last)
                .and_then(|m| m.into_inner().ok())
                .map(|n| n.value)
        } else {
            None
        }
    }

    fn append(&mut self, l: Self) {
        if let Some(last) = self.last.take() {
            last.lock().map(|mut old_last| {
                old_last.next = l.first.clone();
            }).unwrap();
        } else {
            self.first = l.first.clone();
        }
        self.len += l.len;
        self.last = l.last.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_works() {
        let mut l = DoublyLinkedList::<usize>::new();
        assert_eq!(l.len, 0);
        assert!(l.first.is_none());
        assert!(l.last.is_none());
        assert_eq!(l.pop_front(), None);
        l.push_front(5);
        assert_eq!(l.len, 1);
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.len, 0);
        l.push_back(10);
        assert_eq!(l.len, 1);
        assert_eq!(l.pop_back(), Some(10));
        assert_eq!(l.len, 0);
        l.push_front(5); // 5
        l.push_back(10); // 5 10
        l.push_front(3); // 3 5 10
        l.push_back(7); // 3 5 10 7
        assert_eq!(l.pop_front(), Some(3));
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), Some(10));
        assert_eq!(l.pop_front(), Some(7));
        assert_eq!(l.len, 0);
        l.push_front(5); // 5
        l.push_back(10); // 5 10
        l.push_front(3); // 3 5 10
        l.push_back(7); // 3 5 10 7
        assert_eq!(l.pop_back(), Some(7));
        assert_eq!(l.pop_back(), Some(10));
        assert_eq!(l.pop_back(), Some(5));
        assert_eq!(l.pop_back(), Some(3));
        assert_eq!(l.len, 0);
        l.push_front(5); // 5
        l.push_back(10); // 5 10
        let mut l2 = DoublyLinkedList::<usize>::new();
        l2.push_front(3); // 3
        l2.push_back(7); // 3 7
        l.append(l2); // 5 10 3 7
        assert_eq!(l.len, 4);
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), Some(10));
        assert_eq!(l.pop_front(), Some(3));
        assert_eq!(l.pop_front(), Some(7));



    }
}

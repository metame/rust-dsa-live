#![allow(dead_code)]

use std::ops::{Index, IndexMut};
use std::ptr;

use crate::rawvec::RawVec;

pub struct ArrayList<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> ArrayList<T> {
    pub fn new() -> Self {
        ArrayList {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        if self.len == self.buf.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::copy(self.ptr_at_offset(0), self.ptr_at_offset(1), self.len);
            ptr::write(self.ptr_at_offset(0), val);
        }

        self.len += 1;

    }

    pub fn push(&mut self, val: T) {
        if self.len == self.buf.cap() {
            self.buf.grow();
        }

        unsafe {
            let p = self.buf.ptr.as_ptr().add(self.len);
            ptr::write(p, val);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            unsafe {
                let p = self.buf.ptr.as_ptr().add(self.len);
                Some(ptr::read(p))
            }
        } else {
            None
        }
    }

    fn ptr_at_offset(&self, offset: usize) -> *mut T {
        unsafe {
            self.buf.ptr.as_ptr().add(offset)
        }
    }
}

impl<T> Index<usize> for ArrayList<T> {
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a T {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        unsafe {
            let p = self.buf.ptr.as_ptr().add(index);
            &*p
        }
    }
}

impl<T> IndexMut<usize> for ArrayList<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        unsafe {
            let p = self.buf.ptr.as_ptr().add(index);
            &mut *p
        }
    }
}

impl<T> IntoIterator for ArrayList<T> {
    type Item = T;
    type IntoIter = ArrayListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayListIter::new(self)
    }
}

pub struct ArrayListIter<T> {
    ptr: *mut T,
    list: ArrayList<T>,
}

impl<T> ArrayListIter<T> {
    fn new(list: ArrayList<T>) -> Self {
        ArrayListIter {
            ptr: list.buf.ptr.as_ptr(),
            list,
        }
    }
}

impl<T> Iterator for ArrayListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.len > 0 {
            let n = unsafe { ptr::read(self.ptr) };

            self.list.len -= 1;
            if self.list.len > 0 {
                self.ptr = unsafe { self.ptr.offset(1) };
            }

            Some(n)
        } else {
            None
        }
    }
}

impl<T> Drop for ArrayListIter<T> {
    fn drop(&mut self) {
        while let Some(v) = self.next() {
            drop(v);
        }
    }
}

impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        while let Some(v) = self.pop() {
            drop(v);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn array_list_works() {
        let mut l = ArrayList::<u8>::new();
        assert_eq!(l.len, 0);
        assert_eq!(l.pop(), None);
        l.push(5);
        assert_eq!(l.len, 1);
        l.push_front(10);
        assert_eq!(l[0], 10);
        assert_eq!(l[1], 5);
        assert_eq!(l.len, 2);
        assert_eq!(l.pop(), Some(5));
        assert_eq!(l.pop(), Some(10));
        l.push(10);
        l.push(254);
        l.push(7);
        l.push(8);
        l.push(9);
        l.push(6);
        assert_eq!(l[2], 7);
        l[2] = 25;
        assert_eq!(l[2], 25);
        assert_eq!(l[5], 6);
        assert_eq!(l.len, 6);
        assert_eq!(l.into_iter().collect::<Vec<u8>>(), vec![10, 254, 25, 8, 9, 6]);
    }

    #[test]
    fn string_list_works() {
        let mut l = ArrayList::<String>::new();

        l.push("hey".to_string());
        l.push("you".to_string());
        l.push("guys".to_string());

        assert_eq!(l.len, 3);
        assert_eq!(l.pop(), Some("guys".to_string()));
        let mut i = l.into_iter();
        assert_eq!(i.next(), Some("hey".to_string()));
    }

}

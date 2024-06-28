#![allow(dead_code)]

use std::ops::{Index, IndexMut};
use std::ptr;

use crate::rawvec::RawVec;

pub struct RingBuffer<T> {
    head: usize,
    buf: RawVec<T>,
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(cap: usize) -> Self {
        RingBuffer {
            head: 0,
            buf: RawVec::<T>::with_capacity(cap),
            len: 0,
        }
    }

    fn wrapped_index(&self, i: usize) -> usize {
        (self.head + i) % self.buf.cap()
    }

    pub fn push_back(&mut self, val: T) {
        let i = self.wrapped_index(self.len);

        unsafe {
            let p = self.buf.ptr.as_ptr().add(i);

            if self.len == self.buf.cap() {
                ptr::replace(p, val);
            } else {
                self.len += 1;
                ptr::write(p, val);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let i = self.head;
        self.head = self.wrapped_index(1);
        self.len -= 1;

        unsafe {
            Some(ptr::read(self.buf.ptr.as_ptr().add(i)))
        }
    }
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a T {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        let i = self.wrapped_index(index);

        unsafe {
            let p = self.buf.ptr.as_ptr().add(i);
            &*p
        }
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        let i = self.wrapped_index(index);

        unsafe {
            let p = self.buf.ptr.as_ptr().add(i);
            &mut *p
        }
    }
}

impl<T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer_works() {
        let mut rb = RingBuffer::<u8>::with_capacity(8);
        assert_eq!(rb.len, 0);
        assert_eq!(rb.pop_front(), None);
        rb.push_back(1);
        assert_eq!(rb[0], 1);
        rb.push_back(9);
        rb.push_back(7);
        rb[2] = 14;
        assert_eq!(rb.len, 3);
        assert_eq!(rb.pop_front(), Some(1));
        assert_eq!(rb.head, 1);
        assert_eq!(rb.pop_front(), Some(9));
        assert_eq!(rb.pop_front(), Some(14));
        assert_eq!(rb.pop_front(), None);
    }

    #[test]
    fn string_ring_works() {
        let mut rb = RingBuffer::<String>::with_capacity(2);
        rb.push_back("Hey".to_string());
        rb[0] = "META".to_string();
        rb.push_back("You".to_string());
        assert_eq!(rb.len, 2);
        rb.push_back("Guys".to_string());
        assert_eq!(rb.len, 2);
        assert_eq!(rb.pop_front(), Some("Guys".to_string()));
    }
}

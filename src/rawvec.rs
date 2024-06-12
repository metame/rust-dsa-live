#![allow(dead_code)]

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::NonNull;

pub struct RawVec<T> {
    pub ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        RawVec {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut rv = Self::new();

        rv.grow_to_cap(cap, None);

        rv
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    fn grow_to_cap(&mut self, new_cap: usize, old_layout: Option<Layout>) {
        let layout = Layout::array::<T>(new_cap).unwrap();

        if let Some(old_layout) = old_layout {
            unsafe {
                let ptr = realloc(self.ptr.as_ptr() as *mut u8, old_layout, layout.size());
                self.ptr = NonNull::new(ptr as *mut T).unwrap();
            }
        } else {
            unsafe {
                let ptr = alloc(layout);
                self.ptr = NonNull::new(ptr as *mut T).unwrap();
            }
        }

        self.cap = new_cap;
    }

    pub fn grow(&mut self) {
        let (new_cap, old_layout) = if self.cap == 0 {
            (5, None)
        } else {
            let layout = Layout::array::<T>(self.cap).unwrap();
            (self.cap * 2, Some(layout))
        };

        self.grow_to_cap(new_cap, old_layout);
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn rawvec_works() {
        let mut rv = RawVec::<i64>::new();
        assert_eq!(rv.cap, 0);
        rv.grow();
        assert_eq!(rv.cap, 5);
        let ptr = &mut rv.ptr;
        unsafe {
            std::ptr::write(ptr.as_ptr(), 50);
            assert_eq!(*ptr.as_ptr(), 50);
        }

        rv.grow();
        assert_eq!(rv.cap, 10);
        let ptr = &mut rv.ptr;
        unsafe {
            assert_eq!(*ptr.as_ptr(), 50);
            let p = ptr.as_ptr().add(9);
            std::ptr::write(p, 67);
            assert_eq!(*p, 67);
        }

        let ptr = rv.ptr.as_ptr();

        drop(rv);

        // this isn't a great test but passes for now
        unsafe {
            assert_ne!(*ptr, 50);
        }
    }
}

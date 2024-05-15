#![allow(dead_code)]
/**
Linear search and binary search
Because of using recursive operations on slices and lack of TCO in Rust,
the functions operating on slices can stack overflow on large slices
*/
fn linear_search<T, U, P>(mut it: U, p: P) -> Option<T>
where
    T: PartialEq,
    U: Iterator<Item = T>,
    P: Fn(&T) -> bool,
{
    match it.next() {
        Some(a) if p(&a) => Some(a),
        Some(_) => linear_search(it, p),
        None => None,
    }
}

fn linear_search_slice<T: PartialEq>(s: &[T], q: &T) -> Option<usize> {
    do_linear_search_slice(s, q, 0)
}

fn do_linear_search_slice<T: PartialEq>(s: &[T], q: &T, i: usize) -> Option<usize> {
    if i < s.len() {
        if &s[i] == q {
            Some(i)
        } else {
            do_linear_search_slice(s, q, i + 1)
        }
    } else {
        None
    }
}

fn binary_search<T>(s: &[T], q: &T) -> Option<usize>
where
    T: PartialOrd + PartialEq + std::fmt::Debug,
{
    do_binary_search(s, q, 0, s.len())
}

fn do_binary_search<T>(s: &[T], q: &T, low: usize, high: usize) -> Option<usize>
where
    T: PartialOrd + PartialEq,
{
    if low >= high {
        return None;
    }

    let mid = (high + low) / 2;

    if &s[mid] == q {
        Some(mid)
    } else if q < &s[mid] {
        do_binary_search(s, q, low, mid)
    } else {
        do_binary_search(s, q, mid + 1, high)
    }
}

fn binary_search_rec<T>(s: &[T], q: &T) -> Option<usize>
where
    T: PartialOrd + PartialEq,
{
    let mid = s.len() / 2;

    if &s[mid] == q {
        Some(mid)
    } else if q < &s[mid] && s.len() > 1 {
        binary_search_rec(&s[..mid], q)
    } else if q > &s[mid] && s.len() > 1 && (mid + 1) < s.len() {
        binary_search_rec(&s[mid + 1..], q).and_then(|i| Some(i + mid + 1))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_search_works() {
        let it = vec![1, 2, 3].into_iter();
        assert_eq!(Some(3), linear_search(it, |&n| n == 3));
        let it = vec![1, 2, 3].into_iter();
        assert_eq!(None, linear_search(it, |&n| n == 4));
    }

    #[test]
    fn linear_search_slice_works() {
        let s = [1, 2, 3].as_slice();
        assert_eq!(Some(2), linear_search_slice(s, &3));
        assert_eq!(None, linear_search_slice(s, &4));
    }

    #[test]
    fn binary_search_works() {
        let s = [1, 3, 5, 8, 13, 21].as_slice();
        assert_eq!(Some(1), binary_search(s, &3));
        assert_eq!(Some(4), binary_search(s, &13));
        assert_eq!(Some(5), binary_search(s, &21));
        assert_eq!(None, binary_search(s, &2));
        assert_eq!(None, binary_search(s, &0));
        assert_eq!(None, binary_search(s, &23));
    }

    #[test]
    fn binary_search_rec_works() {
        let s = [1, 3, 5, 8, 13, 21].as_slice();
        assert_eq!(Some(0), binary_search_rec(s, &1));
        assert_eq!(Some(1), binary_search_rec(s, &3));
        assert_eq!(Some(2), binary_search_rec(s, &5));
        assert_eq!(Some(3), binary_search_rec(s, &8));
        assert_eq!(Some(4), binary_search_rec(s, &13));
        assert_eq!(Some(5), binary_search_rec(s, &21));
        assert_eq!(None, binary_search_rec(s, &0));
        assert_eq!(None, binary_search_rec(s, &2));
        assert_eq!(None, binary_search_rec(s, &15));
        assert_eq!(None, binary_search_rec(s, &23));
    }
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_dsa::queue::{ArrayQueue, LinkedQueue, StdArrayQueue, StdLinkedQueue};

fn array_queue(size: usize) {
    let mut s = ArrayQueue::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn linked_queue(size: usize) {
    let mut s = LinkedQueue::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn std_array_queue(size: usize) {
    let mut s = StdArrayQueue::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn std_linked_queue(size: usize) {
    let mut s = StdLinkedQueue::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn queue_bench(c: &mut Criterion) {
    c.bench_function("ArrayQueue", |b| b.iter(|| array_queue(black_box(50))));
    c.bench_function("LinkedQueue", |b| b.iter(|| linked_queue(black_box(50))));
    c.bench_function("StdArrayQueue", |b| b.iter(|| std_array_queue(black_box(50))));
    c.bench_function("StdLinkedQueue", |b| b.iter(|| std_linked_queue(black_box(50))));
}

criterion_group!(benches, queue_bench);
criterion_main!(benches);

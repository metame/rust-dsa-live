use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_dsa::stack::{ArrayStack, LinkedStack, StdArrayStack, StdLinkedStack};

fn array_stack(size: usize) {
    let mut s = ArrayStack::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn linked_stack(size: usize) {
    let mut s = LinkedStack::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn std_array_stack(size: usize) {
    let mut s = StdArrayStack::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn std_linked_stack(size: usize) {
    let mut s = StdLinkedStack::<usize>::new();
    for n in 0..size {
        s.push(n);
    }
    while let Some(_) = s.pop() {}
}

fn stack_bench(c: &mut Criterion) {
    c.bench_function("ArrayStack", |b| b.iter(|| array_stack(black_box(50))));
    c.bench_function("LinkedStack", |b| b.iter(|| linked_stack(black_box(50))));
    c.bench_function("StdArrayStack", |b| b.iter(|| std_array_stack(black_box(50))));
    c.bench_function("StdLinkedStack", |b| b.iter(|| std_linked_stack(black_box(50))));
}

criterion_group!(benches, stack_bench);
criterion_main!(benches);

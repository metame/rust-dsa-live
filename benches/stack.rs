use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
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
    let mut group = c.benchmark_group("Stack");
    for size in [25, 50, 100].iter() {

        group.bench_with_input(BenchmarkId::new("ArrayStack", size), size, |b, size| b.iter(|| array_stack(*size)));
        group.bench_with_input(BenchmarkId::new("LinkedStack", size), size, |b, size| b.iter(|| linked_stack(*size)));
        group.bench_with_input(BenchmarkId::new("StdArrayStack", size), size, |b, size| b.iter(|| std_array_stack(*size)));
        group.bench_with_input(BenchmarkId::new("StdLinkedStack", size), size, |b, size| b.iter(|| std_linked_stack(*size)));
    }
}

criterion_group!(benches, stack_bench);
criterion_main!(benches);

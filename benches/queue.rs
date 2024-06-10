use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
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
    let mut group = c.benchmark_group("Queue");
    for size in [25, 50, 100].iter() {

        group.bench_with_input(BenchmarkId::new("ArrayQueue", size), size, |b, size| b.iter(|| array_queue(*size)));
        group.bench_with_input(BenchmarkId::new("LinkedQueue", size), size, |b, size| b.iter(|| linked_queue(*size)));
        group.bench_with_input(BenchmarkId::new("StdArrayQueue", size), size, |b, size| b.iter(|| std_array_queue(*size)));
        group.bench_with_input(BenchmarkId::new("StdLinkedQueue", size), size, |b, size| b.iter(|| std_linked_queue(*size)));
    }
}

criterion_group!(benches, queue_bench);
criterion_main!(benches);

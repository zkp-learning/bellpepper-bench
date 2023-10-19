use std::iter::Iterator;

use bellpepper_core_modify::{
    Index as IndexM, LinearCombination as LinearCombinationM, Variable as VariableM,
};
use bellpepper_core_origin::{Index, LinearCombination, Variable};
use blstrs::Scalar as Fr;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ff::Field;
use rand::thread_rng;
use reservoir::sample;

fn lc_add_fr_origin<T: Iterator<Item = usize>>(items: T) {
    let mut lc = LinearCombination::<Fr>::zero();
    for i in items {
        let coeff = Fr::ONE;
        lc = lc + (coeff, Variable::new_unchecked(Index::Aux(i)));
    }
}

fn lc_add_fr_modify<T: Iterator<Item = usize>>(items: T) {
    let mut lc = LinearCombinationM::<Fr>::zero();
    for i in items {
        let coeff = Fr::ONE;
        lc = lc + (coeff, VariableM::new_unchecked(IndexM::Aux(i)));
    }
}

fn new_lc_origin<T: Iterator<Item = usize>>(items: T, a: usize) -> LinearCombination<Fr> {
    let mut lc = LinearCombination::<Fr>::zero();
    let coeff = Fr::ONE;
    for i in items {
        lc = lc + (coeff, Variable::new_unchecked(Index::Aux(i * a)));
    }
    lc
}

fn new_lc_modify<T: Iterator<Item = usize>>(items: T, a: usize) -> LinearCombinationM<Fr> {
    let mut lc = LinearCombinationM::<Fr>::zero();
    let coeff = Fr::ONE;
    for i in items {
        lc = lc + (coeff, VariableM::new_unchecked(IndexM::Aux(i * a)));
    }
    lc
}

fn lc_add_lc_origin<T: Iterator<Item = usize>>(
    items: T,
    mut lc1: LinearCombination<Fr>,
    lc2: LinearCombination<Fr>,
) {
    for _ in items {
        lc1 = lc1 + &lc2;
    }
}

fn lc_add_lc_modify<T: Iterator<Item = usize>>(
    items: T,
    mut lc1: LinearCombinationM<Fr>,
    lc2: LinearCombinationM<Fr>,
) {
    for _ in items {
        lc1 = lc1 + &lc2;
    }
}

fn bench_add_fr_order(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_fr_order");
    for i in 10..12u64 {
        group.bench_with_input(BenchmarkId::new("origin", i), &i, |b, _| {
            b.iter(|| lc_add_fr_origin(black_box(0..100)))
        });
        group.bench_with_input(BenchmarkId::new("modify", i), &i, |b, _| {
            b.iter(|| lc_add_fr_modify(black_box(0..100)))
        });
    }
    group.finish();
}

fn bench_add_fr_rev_order(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_fr_rev_order");
    for i in 10..12u64 {
        group.bench_with_input(BenchmarkId::new("origin", i), &i, |b, _| {
            b.iter(|| lc_add_fr_origin(black_box((0..100).rev())))
        });
        group.bench_with_input(BenchmarkId::new("modify", i), &i, |b, _| {
            b.iter(|| lc_add_fr_modify(black_box((0..100).rev())))
        });
    }
    group.finish();
}

fn bench_add_fr_disorder(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_fr_disorder");
    let items = sample(&mut thread_rng(), 50, 0..100usize).into_iter();
    for i in 10..12u64 {
        group.bench_with_input(BenchmarkId::new("origin", i), &i, |b, _| {
            b.iter(|| lc_add_fr_origin(items.clone()))
        });
        group.bench_with_input(BenchmarkId::new("modify", i), &i, |b, _| {
            b.iter(|| lc_add_fr_modify(items.clone()))
        });
    }
    group.finish();
}

fn bench_add_lc(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_lc");
    let lc_origin1 = new_lc_origin(0..10, 1);
    let lc_origin2 = new_lc_origin(0..10, 2);
    let lc_modify1 = new_lc_modify(0..10, 1);
    let lc_modify2 = new_lc_modify(0..10, 2);

    for i in 10..12u64 {
        group.bench_with_input(BenchmarkId::new("origin", i), &i, |b, _| {
            b.iter(|| lc_add_lc_origin(black_box(0..10), lc_origin1.clone(), lc_origin2.clone()))
        });
        group.bench_with_input(BenchmarkId::new("modify", i), &i, |b, _| {
            b.iter(|| lc_add_lc_modify(black_box(0..10), lc_modify1.clone(), lc_modify2.clone()))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_add_fr_order,
    bench_add_fr_rev_order,
    bench_add_fr_disorder,
    bench_add_lc
);
criterion_main!(benches);

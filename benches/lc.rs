use std::iter::Iterator;

use bellpepper_core_btreemap::{
    Index as IndexB, LinearCombination as LinearCombinationB, Variable as VariableB,
};
use bellpepper_core_hashmap::{
    Index as IndexH, LinearCombination as LinearCombinationH, Variable as VariableH,
};
use bellpepper_core_indexmap::{
    Index as IndexI, LinearCombination as LinearCombinationI, Variable as VariableI,
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

fn lc_add_fr_btreemap<T: Iterator<Item = usize>>(items: T) {
    let mut lc = LinearCombinationB::<Fr>::zero();
    for i in items {
        let coeff = Fr::ONE;
        lc = lc + (coeff, VariableB::new_unchecked(IndexB::Aux(i)));
    }
}

fn lc_add_fr_indexmap<T: Iterator<Item = usize>>(items: T) {
    let mut lc = LinearCombinationI::<Fr>::zero();
    for i in items {
        let coeff = Fr::ONE;
        lc = lc + (coeff, VariableI::new_unchecked(IndexI::Aux(i)));
    }
}

fn lc_add_fr_hashmap<T: Iterator<Item = usize>>(items: T) {
    let mut lc = LinearCombinationH::<Fr>::zero();
    for i in items {
        let coeff = Fr::ONE;
        lc = lc + (coeff, VariableH::new_unchecked(IndexH::Aux(i)));
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

fn new_lc_btreemap<T: Iterator<Item = usize>>(items: T, a: usize) -> LinearCombinationB<Fr> {
    let mut lc = LinearCombinationB::<Fr>::zero();
    let coeff = Fr::ONE;
    for i in items {
        lc = lc + (coeff, VariableB::new_unchecked(IndexB::Aux(i * a)));
    }
    lc
}

fn new_lc_indexmap<T: Iterator<Item = usize>>(items: T, a: usize) -> LinearCombinationI<Fr> {
    let mut lc = LinearCombinationI::<Fr>::zero();
    let coeff = Fr::ONE;
    for i in items {
        lc = lc + (coeff, VariableI::new_unchecked(IndexI::Aux(i * a)));
    }
    lc
}

fn new_lc_hashmap<T: Iterator<Item = usize>>(items: T, a: usize) -> LinearCombinationH<Fr> {
    let mut lc = LinearCombinationH::<Fr>::zero();
    let coeff = Fr::ONE;
    for i in items {
        lc = lc + (coeff, VariableH::new_unchecked(IndexH::Aux(i * a)));
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

fn lc_add_lc_btreemap<T: Iterator<Item = usize>>(
    items: T,
    mut lc1: LinearCombinationB<Fr>,
    lc2: LinearCombinationB<Fr>,
) {
    for _ in items {
        lc1 = lc1 + &lc2;
    }
}

fn lc_add_lc_indexmap<T: Iterator<Item = usize>>(
    items: T,
    mut lc1: LinearCombinationI<Fr>,
    lc2: LinearCombinationI<Fr>,
) {
    for _ in items {
        lc1 = lc1 + &lc2;
    }
}

fn lc_add_lc_hashmap<T: Iterator<Item = usize>>(
    items: T,
    mut lc1: LinearCombinationH<Fr>,
    lc2: LinearCombinationH<Fr>,
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
        group.bench_with_input(BenchmarkId::new("btreemap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_btreemap(black_box(0..100)))
        });
        group.bench_with_input(BenchmarkId::new("indexmap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_indexmap(black_box(0..100)))
        });
        group.bench_with_input(BenchmarkId::new("hashmap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_hashmap(black_box(0..100)))
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
        group.bench_with_input(BenchmarkId::new("btreemap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_btreemap(black_box((0..100).rev())))
        });
        group.bench_with_input(BenchmarkId::new("indexmap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_indexmap(black_box((0..100).rev())))
        });
        group.bench_with_input(BenchmarkId::new("hashmap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_hashmap(black_box((0..100).rev())))
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
        group.bench_with_input(BenchmarkId::new("btreemap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_btreemap(items.clone()))
        });
        group.bench_with_input(BenchmarkId::new("indexmap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_indexmap(items.clone()))
        });
        group.bench_with_input(BenchmarkId::new("hashmap", i), &i, |b, _| {
            b.iter(|| lc_add_fr_hashmap(items.clone()))
        });
    }
    group.finish();
}

fn bench_add_lc(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_lc");
    let lc_origin1 = new_lc_origin(0..10, 1);
    let lc_origin2 = new_lc_origin(0..10, 2);
    let lc_btreemap1 = new_lc_btreemap(0..10, 1);
    let lc_btreemap2 = new_lc_btreemap(0..10, 2);
    let lc_indexmap1 = new_lc_indexmap(0..10, 1);
    let lc_indexmap2 = new_lc_indexmap(0..10, 2);
    let lc_hashmap1 = new_lc_hashmap(0..10, 1);
    let lc_hashmap2 = new_lc_hashmap(0..10, 2);

    for i in 10..12u64 {
        group.bench_with_input(BenchmarkId::new("origin", i), &i, |b, _| {
            b.iter(|| lc_add_lc_origin(black_box(0..10), lc_origin1.clone(), lc_origin2.clone()))
        });
        group.bench_with_input(BenchmarkId::new("btreemap", i), &i, |b, _| {
            b.iter(|| {
                lc_add_lc_btreemap(black_box(0..10), lc_btreemap1.clone(), lc_btreemap2.clone())
            })
        });
        group.bench_with_input(BenchmarkId::new("indexmap", i), &i, |b, _| {
            b.iter(|| {
                lc_add_lc_indexmap(black_box(0..10), lc_indexmap1.clone(), lc_indexmap2.clone())
            })
        });
        group.bench_with_input(BenchmarkId::new("hashmap", i), &i, |b, _| {
            b.iter(|| lc_add_lc_hashmap(black_box(0..10), lc_hashmap1.clone(), lc_hashmap2.clone()))
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

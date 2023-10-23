use std::collections::BTreeMap;

use indexmap::IndexMap;
use rand::thread_rng;
use reservoir::sample;

fn main() {
    let samples = sample(&mut thread_rng(), 50, 0..100);

    for i in samples {
        print!("{} ", i);
    }
    println!("");

    let mut map = IndexMap::new();
    let mut bt = BTreeMap::new();
    for i in (0..20).rev() {
        map.insert(i, 0);
        bt.insert(i, 0);
    }
    map.sort_keys();
    for i in map.iter() {
        println!("{:?}", i);
    }
}

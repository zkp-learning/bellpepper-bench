use rand::thread_rng;
use reservoir::sample;

fn main() {
    let samples = sample(&mut thread_rng(), 50, 0..100);

    for i in samples {
        println!("{}", i);
    }
}

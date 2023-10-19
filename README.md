# bellpepper-bench

## the Indexer:

| Condition                | Vec + pointer | BTreeMap |
| ------------------------ | ------------- | -------- |
| add Fr with order        | 853.55 ns     | 2.8 us   |
| add Fr with rev order    | 3.9 us        | 3.2 us   |
| add Fr with random order | 1.1 us        | 1.4 us   |
| add lc                   | 777 ns        | 1.4 us   |





detail:
```sh
cargo bench
   Compiling bellpepper-bench v0.1.0 (/Users/flyq/workspace/github/zkp-learning/bellpepper-bench)
    Finished bench [optimized] target(s) in 1.26s
     Running unittests src/main.rs (target/release/deps/bellpepper_bench-4fca748cf49192e5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/lc.rs (target/release/deps/lc-0bd14802c91034b5)
add_fr_order/origin/10  time:   [835.19 ns 853.55 ns 884.80 ns]
                        change: [+0.1349% +1.5085% +3.7411%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
add_fr_order/modify/10  time:   [2.8113 µs 2.8198 µs 2.8282 µs]
                        change: [-0.6371% -0.3609% -0.0926%] (p = 0.01 < 0.05)
                        Change within noise threshold.
add_fr_order/origin/11  time:   [858.72 ns 864.54 ns 870.15 ns]
add_fr_order/modify/11  time:   [2.7868 µs 2.7932 µs 2.8002 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

add_fr_rev_order/origin/10
                        time:   [3.8774 µs 3.9094 µs 3.9413 µs]
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high mild
add_fr_rev_order/modify/10
                        time:   [3.2907 µs 3.2973 µs 3.3035 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
add_fr_rev_order/origin/11
                        time:   [3.8278 µs 3.8655 µs 3.9056 µs]
add_fr_rev_order/modify/11
                        time:   [3.2885 µs 3.2976 µs 3.3067 µs]

add_fr_disorder/origin/10
                        time:   [1.1516 µs 1.1560 µs 1.1604 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
add_fr_disorder/modify/10
                        time:   [1.4224 µs 1.4262 µs 1.4302 µs]
add_fr_disorder/origin/11
                        time:   [1.1440 µs 1.1493 µs 1.1545 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) low mild
  1 (1.00%) high mild
add_fr_disorder/modify/11
                        time:   [1.4273 µs 1.4301 µs 1.4330 µs]

add_lc/origin/10        time:   [776.11 ns 777.77 ns 779.33 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
add_lc/modify/10        time:   [1.3950 µs 1.3987 µs 1.4030 µs]
add_lc/origin/11        time:   [768.90 ns 771.23 ns 773.66 ns]
add_lc/modify/11        time:   [1.3980 µs 1.4013 µs 1.4046 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
```
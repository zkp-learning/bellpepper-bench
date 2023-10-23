# bellpepper-bench

## the Indexer:

| Condition                | Vec + pointer | BTreeMap | indexMap | hashMap |
| ------------------------ | ------------- | -------- | -------- | ------- |
| add Fr with order        | 824.17 ns     | 2.81 us  | 7.03 us  | 8.82 us |
| add Fr with rev order    | 3.85 us       | 3.34 us  | 7.05 us  | 8.88 us |
| add Fr with random order | 939.47 us     | 1.48 us  | 3.84 us  | 4.67 us |
| add lc                   | 763.49 ns     | 1.23 us  | 3.02 us  | 2.68 us |





detail:
```sh
cargo bench
   Compiling bellpepper-core v0.2.1 (https://github.com/zkp-learning/bellpepper?branch=indexmap#f744ce30)
   Compiling bellpepper-core v0.2.1 (https://github.com/zkp-learning/bellpepper?branch=hashmap#b9d8a75c)
   Compiling bellpepper-bench v0.1.0 (/Users/flyq/workspace/github/zkp-learning/bellpepper-bench)
    Finished bench [optimized] target(s) in 2.43s
     Running unittests src/main.rs (target/release/deps/bellpepper_bench-a11258a0e9b84c14)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/lc.rs (target/release/deps/lc-78dec87322884072)
add_fr_order/origin/10  time:   [821.84 ns 824.17 ns 826.80 ns]
                        change: [-8.4621% -7.8357% -7.2080%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
add_fr_order/btreemap/10
                        time:   [2.7971 µs 2.8054 µs 2.8136 µs]
                        change: [-6.1958% -5.9217% -5.6069%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
add_fr_order/indexmap/10
                        time:   [7.0292 µs 7.0383 µs 7.0475 µs]
                        change: [-7.6066% -7.4474% -7.2787%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
add_fr_order/hashmap/10 time:   [8.7915 µs 8.8250 µs 8.8643 µs]
Found 14 outliers among 100 measurements (14.00%)
  8 (8.00%) high mild
  6 (6.00%) high severe
add_fr_order/origin/11  time:   [832.83 ns 837.15 ns 841.71 ns]
                        change: [-4.9695% -4.2811% -3.6127%] (p = 0.00 < 0.05)
                        Performance has improved.
add_fr_order/btreemap/11
                        time:   [2.7868 µs 2.7956 µs 2.8071 µs]
                        change: [-3.9168% -3.4722% -3.0690%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
add_fr_order/indexmap/11
                        time:   [7.0309 µs 7.0417 µs 7.0534 µs]
                        change: [-4.4110% -4.0531% -3.7427%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
add_fr_order/hashmap/11 time:   [8.8706 µs 8.9183 µs 8.9667 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

add_fr_rev_order/origin/10
                        time:   [3.8168 µs 3.8500 µs 3.8860 µs]
                        change: [-1.6084% -0.9433% -0.2840%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  17 (17.00%) high mild
add_fr_rev_order/btreemap/10
                        time:   [3.3283 µs 3.3358 µs 3.3434 µs]
                        change: [-2.6801% -1.8822% -1.3310%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
add_fr_rev_order/indexmap/10
                        time:   [7.0466 µs 7.0563 µs 7.0663 µs]
                        change: [-1.8128% -1.6129% -1.4058%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
add_fr_rev_order/hashmap/10
                        time:   [8.8295 µs 8.8766 µs 8.9299 µs]
Found 16 outliers among 100 measurements (16.00%)
  14 (14.00%) high mild
  2 (2.00%) high severe
add_fr_rev_order/origin/11
                        time:   [3.8193 µs 3.8471 µs 3.8782 µs]
                        change: [-0.7149% -0.1586% +0.3225%] (p = 0.60 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe
add_fr_rev_order/btreemap/11
                        time:   [3.3087 µs 3.3167 µs 3.3246 µs]
                        change: [-1.3558% -1.1189% -0.8754%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
add_fr_rev_order/indexmap/11
                        time:   [7.0374 µs 7.0409 µs 7.0445 µs]
                        change: [-2.0801% -1.7597% -1.4451%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  12 (12.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
add_fr_rev_order/hashmap/11
                        time:   [8.7846 µs 8.8234 µs 8.8653 µs]

add_fr_disorder/origin/10
                        time:   [931.46 ns 939.47 ns 947.19 ns]
                        change: [-14.543% -13.917% -13.331%] (p = 0.00 < 0.05)
                        Performance has improved.
add_fr_disorder/btreemap/10
                        time:   [1.4745 µs 1.4809 µs 1.4876 µs]
                        change: [-7.0809% -6.6825% -6.3167%] (p = 0.00 < 0.05)
                        Performance has improved.
add_fr_disorder/indexmap/10
                        time:   [3.8312 µs 3.8396 µs 3.8479 µs]
                        change: [-2.6917% -2.4544% -2.2131%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
add_fr_disorder/hashmap/10
                        time:   [4.6639 µs 4.6661 µs 4.6686 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
add_fr_disorder/origin/11
                        time:   [937.85 ns 947.02 ns 956.58 ns]
                        change: [-12.922% -12.211% -11.512%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) low mild
  11 (11.00%) high mild
  5 (5.00%) high severe
add_fr_disorder/btreemap/11
                        time:   [1.4601 µs 1.4628 µs 1.4655 µs]
                        change: [-8.4378% -8.1398% -7.8532%] (p = 0.00 < 0.05)
                        Performance has improved.
add_fr_disorder/indexmap/11
                        time:   [3.8505 µs 3.8604 µs 3.8708 µs]
                        change: [-1.8840% -1.5935% -1.3211%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
add_fr_disorder/hashmap/11
                        time:   [4.6559 µs 4.6612 µs 4.6660 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) low severe
  3 (3.00%) high mild
  1 (1.00%) high severe

add_lc/origin/10        time:   [760.57 ns 763.49 ns 766.24 ns]
                        change: [-3.4801% -3.0745% -2.6739%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
add_lc/btreemap/10      time:   [1.2255 µs 1.2294 µs 1.2337 µs]
                        change: [-1.6580% -1.3676% -1.1053%] (p = 0.00 < 0.05)
                        Performance has improved.
add_lc/indexmap/10      time:   [3.0152 µs 3.0184 µs 3.0220 µs]
                        change: [-1.3422% -1.0179% -0.7343%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
add_lc/hashmap/10       time:   [2.6697 µs 2.6826 µs 2.6946 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) low mild
add_lc/origin/11        time:   [765.45 ns 767.40 ns 769.13 ns]
                        change: [-2.3247% -2.0154% -1.7041%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
add_lc/btreemap/11      time:   [1.2171 µs 1.2206 µs 1.2242 µs]
                        change: [-2.4148% -2.1614% -1.9012%] (p = 0.00 < 0.05)
                        Performance has improved.
add_lc/indexmap/11      time:   [3.0179 µs 3.0227 µs 3.0279 µs]
                        change: [-1.5306% -1.3189% -1.1013%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
add_lc/hashmap/11       time:   [2.6455 µs 2.6641 µs 2.6811 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) low mild
```
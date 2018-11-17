#![feature(test)]
#![feature(extern_crate_item_prelude)]

//

/*
cargo test --benches
cargo test --bench bench1 -- --nocapture
*/

extern crate find_longest_substring;
extern crate test;

#[cfg(test)]
mod tests {
    use find_longest_substring::longest_substring;
    use test::Bencher;

    #[bench]
    fn bench_100_ints(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(100);
            (0..n).for_each(|x| {
                longest_substring(x.to_string().as_str());
            });
        });
    }

    #[bench]
    fn bench_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1000);
            (0..n).for_each(|x| {
                longest_substring(x.to_string().as_str());
            });
        });
    }

    #[bench]
    fn bench_10000_ints(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(10000);
            (0..n).for_each(|x| {
                longest_substring(x.to_string().as_str());
            });
        });
    }

}

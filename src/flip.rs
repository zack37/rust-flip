use rand;
use rand::{Rng, StdRng};

pub fn flip_coins(n: usize) -> (usize, usize) {
    // Small optimization discovered during benchmark testing
    if n < 10_000 {
        return _flip_coins_(n);
    }
    let mut rng = StdRng::new().expect("OS Random not initialized");
    rng.gen_iter()
        .take(n)
        .fold((0_usize, 0_usize), |acc, cur| if cur {
            (acc.0 + 1, acc.1)
        } else {
            (acc.0, acc.1 + 1)
        })
}

pub fn flip_best_of(n: usize) -> (usize, usize) {
    let mut rng = StdRng::new().expect("OS Random not initialized");
    // best of 7 = 4, best of 8 = 5
    let half = ((n + 1) as f64 / 2.0).ceil() as usize;

    let (mut head_count, mut tail_count) = (0, 0);
    while head_count < half && tail_count < half {
        if rng.gen() {
            head_count += 1;
        } else {
            tail_count += 1;
        }
    }
    (head_count, tail_count)
}

fn _flip_coins_(n: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let mut head_count = 0;
    let mut tail_count = 0;
    for _ in 0..n {
        if rng.gen() {
            head_count += 1;
        } else {
            tail_count += 1;
        }
    }
    (head_count, tail_count)
}

#[cfg(test)]
mod bench {

    use super::*;
    use super::_flip_coins_;
    use test::Bencher;

    #[bench]
    fn bench_coin_flips_2000_iterative(b: &mut Bencher) {
        b.iter(|| _flip_coins_(2_000));
    }

    #[bench]
    fn bench_coin_flips_2000_pipeline(b: &mut Bencher) {
        b.iter(|| flip_coins(2_000))
    }

    #[bench]
    fn bench_coin_flips_10000_iterative(b: &mut Bencher) {
        b.iter(|| _flip_coins_(10_000));
    }

    #[bench]
    fn bench_coin_flips_10000_pipeline(b: &mut Bencher) {
        b.iter(|| flip_coins(10_000))
    }

    #[bench]
    fn bench_coin_flips_20000_iterative(b: &mut Bencher) {
        b.iter(|| _flip_coins_(20_000));
    }

    #[bench]
    fn bench_coin_flips_20000_pipeline(b: &mut Bencher) {
        b.iter(|| flip_coins(20_000))
    }

    #[bench]
    fn bench_coin_flips_200000_iterative(b: &mut Bencher) {
        b.iter(|| _flip_coins_(200_000));
    }

    #[bench]
    fn bench_coin_flips_200000_pipeline(b: &mut Bencher) {
        b.iter(|| flip_coins(200_000))
    }

    #[bench]
    fn bench_coin_flips_2000000_iterative(b: &mut Bencher) {
        b.iter(|| _flip_coins_(2_000_000));
    }

    #[bench]
    fn bench_coin_flips_2000000_pipeline(b: &mut Bencher) {
        b.iter(|| flip_coins(2_000_000))
    }

    #[bench]
    fn bench_best_of_2000(b: &mut Bencher) {
        b.iter(|| flip_best_of(2_000))
    }

    #[bench]
    fn bench_best_of_20000(b: &mut Bencher) {
        b.iter(|| flip_best_of(20_000))
    }

    #[bench]
    fn bench_best_of_200000(b: &mut Bencher) {
        b.iter(|| flip_best_of(200_000))
    }

    #[bench]
    fn bench_best_of_2000000(b: &mut Bencher) {
        b.iter(|| flip_best_of(2_000_000))
    }

}

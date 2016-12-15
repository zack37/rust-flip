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
	.fold((0_usize,0_usize), |acc, cur| match cur {
		true => (acc.0+1, acc.1),
		false => (acc.0, acc.1+1)
	})
}

fn _flip_coins_(n: usize) -> (usize, usize) {
	let mut rng = rand::thread_rng();
	let mut head_count = 0;
	let mut tail_count = 0;
	for _ in 0..n {
		if rng.gen() {
			head_count += 1;
		}
		else {
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

}
#![feature(test)]
#[cfg(test)]
extern crate test;

#[macro_use]
extern crate clap;
extern crate rand;

use clap::App;

mod args;
mod flip;

fn main() {
	let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(crate_version!())
		.author(crate_authors!())
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(args::times_arg())
		.arg(args::heads_arg())
		.arg(args::tails_arg())
		.arg(args::best_of_arg())
		.get_matches();

	let (n, heads, tails, best_of_opt) = (
		value_t!(matches, "times", usize).unwrap_or(1),
		matches.value_of("heads").unwrap(),
		matches.value_of("tails").unwrap(),
		value_t!(matches, "best_of", usize)
	);

	let counts = match best_of_opt {
		Ok(best_of) => flip::flip_best_of(best_of),
		_ => flip::flip_coins(n)
	};
	let winner = match counts {
		(h, t) if h > t => heads,
		(h, t) if t > h => tails,
		_ => "TIE"
	};

	let num_flips = best_of_opt.unwrap_or(n);

	let n_flips = format!("{} flip{}", num_flips, match num_flips { 1 => "", _ => "s" });

	println!("The winner after {} is: {}", n_flips, winner);
}

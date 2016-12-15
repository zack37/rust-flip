#![feature(test)]
#[cfg(test)]
extern crate test;

#[macro_use(value_t_or_exit)]
extern crate clap;
extern crate rand;

use clap::App;

mod args;
mod flip;

fn main() {
	let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(args::times_arg())
		.arg(args::heads_arg())
		.arg(args::tails_arg())
		.get_matches();

	let (n, heads, tails) = (
		value_t_or_exit!(matches, "times", usize),
		matches.value_of("heads").unwrap(),
		matches.value_of("tails").unwrap()
	);

	let counts = flip::flip_coins(n);
	let winner = match counts {
		(h, t) if h > t => heads,
		(h, t) if t > h => tails,
		_ => "TIE"
	};

	let n_flips = format!("{} flip{}", n, match n { 1 => "", _ => "s" });

	println!("The winner after {} is: {}", n_flips, winner);
}

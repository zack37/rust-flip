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
	let matches = App::new("rs-flip")
		.version("v1.0-alpha")
		.author("Zack Smith <zack.t.smith37@gmail.com>")
		.about("Flips a coin n times")
		.arg(args::times_arg())
		.arg(args::heads_arg())
		.arg(args::tails_arg())
		.get_matches();

	let (n, heads, tails) = (
		value_t_or_exit!(matches, "times", usize),
		matches.value_of("heads").unwrap(),
		matches.value_of("tails").unwrap()
	);

	let (heads_count, tails_count) = flip::flip_coins(n);
	let winner = if heads_count > tails_count { heads }
		else if tails_count > heads_count { tails }
		else { "TIE" };
	let n_flips = format!("{} flip{}", n, match n { 1 => "", _ => "s" });

	println!("The winner after {} is: {}", n_flips, winner);
}

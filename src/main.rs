#![feature(test)]
#[cfg(test)]
extern crate test;

#[macro_use]
extern crate clap;
extern crate rand;

use clap::App;

mod args;
mod flip;

use args::{times_arg, heads_arg, tails_arg, best_of_arg};
use flip::{flip_coins, flip_best_of};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(times_arg())
        .arg(heads_arg())
        .arg(tails_arg())
        .arg(best_of_arg())
        .get_matches();

    let (n, heads, tails, best_of_opt) = (value_t!(matches, "times", usize).unwrap_or(1),
                                          matches.value_of("heads").unwrap(),
                                          matches.value_of("tails").unwrap(),
                                          value_t!(matches, "best_of", usize));

    let counts = if let Ok(best_of) = best_of_opt {
        flip_best_of(best_of)
    } else {
        flip_coins(n)
    };

    let winner = match counts {
        (h, t) if h > t => heads,
        (h, t) if t > h => tails,
        _ => "TIE",
    };

    let num_flips = best_of_opt.unwrap_or(n);

    let n_flips = format!("{} flip{}",
                          num_flips,
                          match num_flips {
                              1 => "",
                              _ => "s",
                          });

    println!("The winner after {} is: {} with ({:?})",
             n_flips,
             winner,
             counts);
}

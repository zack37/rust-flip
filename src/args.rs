use clap::Arg;

pub fn times_arg() -> Arg<'static, 'static> {
	Arg::with_name("times")
		.short("n")
		.long("times")
		.help("Number of times to flip the coin")
		.takes_value(true)
}

pub fn heads_arg() -> Arg<'static, 'static> {
	Arg::with_name("heads")
		.short("h")
		.long("heads")
		.help("Value for heads")
		.default_value("heads")
}

pub fn tails_arg() -> Arg<'static, 'static> {
	Arg::with_name("tails")
		.short("t")
		.long("tails")
		.help("Value for tails")
		.default_value("tails")
}

pub fn best_of_arg() -> Arg<'static, 'static> {
	Arg::with_name("best_of")
		.short("b")
		.long("best-of")
		.help("Flip until either heads or tails passes half way")
		.takes_value(true)
		.conflicts_with("times")
}

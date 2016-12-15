use clap::Arg;

pub fn times_arg() -> Arg<'static, 'static> {
	Arg::with_name("times")
		.short("n")
		.long("times")
		.help("Number of times to flip the coin")
		.default_value("1")
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

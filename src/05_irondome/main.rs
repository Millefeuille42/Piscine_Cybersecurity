use std::path::PathBuf;
use std::process::exit;
use nix::unistd::Uid;
use clap::{Arg, ArgAction, Command};

fn main() {
	if !Uid::effective().is_root() {
		eprintln!("Error: irondome can only be run as root");
		exit(1);
	}
	// TODO set memory limit

	let matches = Command::new("irondome")
		.arg(Arg::new("foreground")
			.short('f')
			.long("foreground")
			.action(ArgAction::SetTrue)
			.help("start as a foreground program instead of a daemon"))
		.arg(Arg::new("paths")
			.num_args(1..)
			.value_name("PATH")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true)
			.help("List of paths to watch"))
		.get_matches();

	let paths = matches.get_many::<PathBuf>("paths");
	if paths.is_none() {
		eprintln!("Error: no path provided");
		exit(1);
	}
	let paths: Vec<&PathBuf> = paths.unwrap().collect();
	for path in paths {
		println!("{}", path.display());
	}

	if matches.get_flag("foreground") {
		todo!("run app in foreground")
	}

	todo!("run app as a daemon")
}
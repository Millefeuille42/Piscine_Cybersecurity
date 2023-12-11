use std::{fs, io};
use std::path::PathBuf;
use std::process::exit;
use clap::{Arg, ArgAction, Command};
use entropy::shannon_entropy;
use nix::unistd::Uid;
use nix::sys::resource::{Resource, setrlimit};
use inotify::{EventMask, Inotify, WatchMask};

fn get_entropy(path: &PathBuf) -> Result<f32, io::Error> {
	let file_data = fs::read(path)?;
	Ok(shannon_entropy(file_data))
}

fn monitor_read(paths: &Vec<&PathBuf>) {
	let inotify = Inotify::init();
	if let Err(err) = inotify {
		eprintln!("Error: could not start i/o monitoring engine: {}", err);
		return;
	}
	let mut inotify = inotify.unwrap();

	for file in paths {
		if let Err(err) = inotify.watches().add(
			file,
			WatchMask::ACCESS | WatchMask::MODIFY,
		) {
			eprintln!("Error: could not start watch on {:?}: {}", file, err);
		}
	}

	loop {
		let mut buffer = [0; 1024];
		let events = inotify.read_events_blocking(&mut buffer)
			.expect("Error while reading events");

		for event in events {
			println!("{:?}", event);
			let aaa: usize = (event.wd.get_watch_descriptor_id() - 1) as usize;
			let base_path = paths.get(aaa);
			if base_path.is_none() {
				continue;
			}
			let base_path = base_path.unwrap();
			let p = base_path.join(event.name.unwrap_or_default());
			if event.mask == EventMask::MODIFY {
				let e = get_entropy(&p);
				if let Err(err) = e {
					eprintln!("Error: could not compute entropy: {}", err);
					continue;
				}
				println!("{}", e.unwrap());
			}
		}
	}
}

fn irondome(paths: &Vec<&PathBuf>) {
	monitor_read(paths);
}

fn main() {
	if Uid::effective().is_root() {
		eprintln!("Error: irondome can only be run as root");
		exit(1);
	}
	if let Err(err) = setrlimit(Resource::RLIMIT_AS, 104857600, 104857600) {
		eprintln!("Error: could not set memory limit: {}", err);
	}

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

	if matches.get_flag("foreground") {
		irondome(&paths);
		return;
	}

	todo!("run app as a daemon");
}
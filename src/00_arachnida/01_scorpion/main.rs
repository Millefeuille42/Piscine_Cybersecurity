use std::env;
use std::os::linux::fs::MetadataExt;
use std::path::PathBuf;
use std::process::exit;
use exif;
use chrono::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		eprintln!("Error: missing argument FILES");
		exit(1);
	}

	for (index, arg) in args.iter().enumerate() {
		if index == 0 { continue; }
		let mut path = PathBuf::default();
		path.push(arg);
		let file = std::fs::File::open(path.to_str().unwrap());
		if let Err(e) = file {
			eprintln!("Error: {}: {e}", path.to_str().unwrap());
			continue;
		}
		let file = file.unwrap();
		println!("{}", path.to_str().unwrap());
		match file.metadata() {
			Ok(metadata) => {
				let ctime: DateTime<Utc> = DateTime::<Utc>::from_timestamp(
					metadata.st_ctime(),
					metadata.st_ctime_nsec() as u32
				).unwrap();
				let ctime = ctime.with_timezone(&Local);

				println!("	created at {}", ctime);
			}
			Err(e) => eprintln!("Error: {e}")
		}

		let mut bufreader = std::io::BufReader::new(&file);
		let exif_reader = exif::Reader::new();
		match exif_reader.read_from_container(&mut bufreader) {
			Ok(exif) => {
				for f in exif.fields() {
					println!("	{} {} {}",
							 f.tag, f.ifd_num, f.display_value().with_unit(&exif));
				}
			}
			Err(_) => {}//eprintln!("Error: {}: {e}", path.to_str().unwrap())
		}
	}
}
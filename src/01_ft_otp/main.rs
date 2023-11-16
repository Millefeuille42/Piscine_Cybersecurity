use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::PathBuf;
use clap::{Arg, Command};
use hmac::{SimpleHmac, Mac};
use sha2::Sha256;

type HmacSha256 = SimpleHmac<Sha256>;

#[derive(Debug)]
struct KeyFormatError;

impl Error for KeyFormatError {}
impl Display for KeyFormatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "key must be 64 hexadecimal characters.")
	}
}


fn main() {
	let matches = Command::new("00_spider")
		.arg(Arg::new("generate")
			.short('g')
			.exclusive(true)
			.required(true)
			.value_parser(clap::value_parser!(PathBuf))
			.value_name("KEY_FILE")
			.help("Generate keyfile based on a hexadecimal key"))
		.arg(Arg::new("key")
			.short('k')
			.value_name("KEY_FILE")
			.exclusive(true)
			.required(true)
			.value_parser(clap::value_parser!(PathBuf))
			.help("Get OTP based on provided keyfile"))
		.get_matches();

	if let Some(keyfile) = matches.get_one::<PathBuf>("generate") {
		let key = fs::read_to_string(keyfile).expect("Error: Unable to open file");
		if key.len() != 64 {eprintln!("Error: {KeyFormatError}"); return;}
		let key = hex::decode(key).expect(format!("Error: {KeyFormatError}").as_str());

		let mut mac = HmacSha256::new_from_slice(&key).expect("Error: Unable to use key");
		mac.update(b"\0");
		let result = mac.finalize();
		let result = result.into_bytes();
		let result = result.as_slice();
		let result = hex::encode(result);
		println!("{}", result.len());

		return;
	}

	if let Some(keyfile) = matches.get_one::<PathBuf>("key") {
		println!("{}", keyfile.to_str().unwrap());
		return;
	}
}
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::PathBuf;
use clap::{Arg, Command};
use hmac::{SimpleHmac, Mac};
use sha1::Sha1;
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;

type HmacSha1 = SimpleHmac<Sha1>;

#[derive(Debug)]
struct KeyFormatError;

impl Error for KeyFormatError {}
impl Display for KeyFormatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "key must be 64 hexadecimal characters.")
	}
}

fn hotp(key: &[u8], counter: u64) -> u32 {
	let mut hmac = HmacSha1::new_from_slice(key).expect(format!("Error: {KeyFormatError}").as_str());

	let counter: [u8; 8] = counter.to_be_bytes();
	hmac.update(&counter);

	let result = hmac.finalize().into_bytes();
	let offset = (result[result.len() - 1] & 0x0F) as usize;
	let otp = ((u32::from(result[offset]) & 0x7F) << 24)
		| ((u32::from(result[offset + 1]) & 0xFF) << 16)
		| ((u32::from(result[offset + 2]) & 0xFF) << 8)
		| (u32::from(result[offset + 3]) & 0xFF);

	otp % 10u32.pow(6u32)
}

fn totp(k: &[u8], t: u64) -> u32 {
	let utc_now = Utc::now();

	let s_since_epoch = utc_now.timestamp() as u64;

	let time_steps = s_since_epoch / t;
	hotp(k, time_steps)
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
		let key = fs::read_to_string(keyfile).expect("Error: Unable to open key file");
		if key.len() != 64 {eprintln!("Error: {KeyFormatError}"); return;}
		let hex_key = hex::decode(key.clone()).expect(format!("Error: {KeyFormatError}").as_str());
		let hex_key_encoded = general_purpose::STANDARD.encode(&hex_key);

		fs::write("./ft_otp.key", hex_key_encoded).expect("Error: Unable to write key");
		return;
	}

	if let Some(keyfile) = matches.get_one::<PathBuf>("key") {
		let key_encoded = fs::read_to_string(keyfile).expect("Error: Unable to open key file");
		let key = general_purpose::STANDARD.decode(key_encoded).expect("Error: corrupted key");
		let totp_val = totp(&key, 30);
		println!("{:0>6}", totp_val);
		return;
	}
}